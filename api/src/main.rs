// #![deny(warnings)]
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;

use common::db::legacy_db::{insert_frame, read_frames};
use common::frame::FrameJson;
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

struct Connection {
    tx: mpsc::UnboundedSender<Message>,
    user: Arc<User>,
}

// #[derive(Default)]
type Users = Arc<RwLock<HashMap<IpAddr, Arc<User>>>>;

struct User {
    // TODO implement cooldown using ip
    ip: IpAddr,
    cooldown_until: Option<Instant>,
}

struct ConnectionManager {
    next_conn_id: AtomicUsize,
    active_connections: Arc<RwLock<HashMap<usize, Connection>>>,
}

impl ConnectionManager {
    fn new() -> Self {
        Self {
            next_conn_id: AtomicUsize::new(1),
            active_connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn open(&self, users: Users, tx: mpsc::UnboundedSender<Message>, ip: IpAddr) -> usize {
        let mut users_lock = users.write().await;

        let user = match users_lock.get(&ip) {
            Some(user) => user,
            None => {
                let user = User {
                    ip,
                    cooldown_until: None,
                };
                users_lock.insert(ip, Arc::new(user));
                users_lock.get(&ip).unwrap()
            }
        };

        let conn_id = self.next_conn_id.fetch_add(1, Ordering::Relaxed);

        self.active_connections.write().await.insert(
            conn_id,
            Connection {
                tx,
                user: user.clone(),
            },
        );

        conn_id
    }

    async fn close(&self, conn_id: usize) {
        self.active_connections.write().await.remove(&conn_id);
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Server...");

    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let conn_manager = Arc::new(ConnectionManager::new());
    // Turn our "state" into a new Filter...
    let conn_manager = warp::any().map(move || conn_manager.clone());

    let ws = warp::path("ws")
        .and(warp::ws())
        .and(conn_manager)
        .and(users)
        .and(warp::addr::remote())
        .map(
            |ws: warp::ws::Ws,
             conn_manager: Arc<ConnectionManager>,
             users,
             addr: Option<SocketAddr>| {
                ws.on_upgrade(move |socket| {
                    println!("New connection: {:?}", addr);
                    user_connected(socket, conn_manager, addr.unwrap(), users)
                })
            },
        );

    let frames = warp::path("frames")
        .and(warp::get())
        .map(|| warp::reply::json(&read_frames().unwrap()));

    let static_files = warp::fs::dir("../ui/dist");

    let routes = frames.or(ws).or(static_files);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    println!("Server stopped");
}

async fn user_connected(
    ws: WebSocket,
    connections: Arc<ConnectionManager>,
    addr: SocketAddr,
    users: Users,
) {
    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Save the sender in our list of connected users.
    let conn_id = connections.open(users, tx, addr.ip()).await;

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", conn_id, e);
                break;
            }
        };
        println!("{}: {:?}", conn_id, msg);
        user_frame(conn_id, msg.clone(), &connections).await;
        write_frame_db(msg).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    connections.close(conn_id).await;
}

async fn write_frame_db(msg: Message) {
    let s = match msg.to_str() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error converting message to string");
            return;
        }
    };

    let frame: FrameJson = match serde_json::from_str(s) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error converting string to FrameJson: {}", e);
            return;
        }
    };

    match insert_frame(frame) {
        Ok(_) => (),
        Err(e) => eprintln!(
            "Error writing frame to db, please make sure the file exists: {}",
            e
        ),
    };
}

async fn user_frame(conn_id: usize, msg: Message, conn_manager: &ConnectionManager) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    // New message from this user, send it to everyone else (except same uid)...
    for (&other_conn_id, conn) in conn_manager.active_connections.read().await.iter() {
        if conn_id != other_conn_id {
            if let Err(_disconnected) = conn.tx.send(Message::text(msg)) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}
