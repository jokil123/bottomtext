use std::net::SocketAddr;
use std::sync::Arc;

use crate::{users::Users, util::ip_hash};

// use common::packets::{
//     c2s::PacketC2S,
//     s2c::{InitializationData, PacketS2C},
// };

use common::{
    db::legacy_db::insert_frame,
    packets::{
        c2s::PacketC2S,
        s2c::{InitializationData, MessageS2C, PacketS2C},
    },
};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::connection_manager::ConnectionManager;

pub async fn user_connected(
    ws: WebSocket,
    connections: Arc<ConnectionManager>,
    addr: SocketAddr,
    users: Users,
) {
    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages to the websocket...
    let (internal_tx, internal_rx) = mpsc::unbounded_channel::<PacketS2C>();
    let mut internal_rx = UnboundedReceiverStream::new(internal_rx);

    // get the user id by hashing the ip
    // TODO: hashing the ip is not a good way to anonymize users
    let user_id = ip_hash(&addr.ip());

    // Save the sender in our list of connected users.
    let conn_id = connections
        .open(users.clone(), internal_tx, addr.ip(), user_id)
        .await;

    // Send the user the initialization data (their id and country)
    user_ws_tx
        .send(
            PacketS2C::InitializeUser(
                InitializationData {
                    user_id,
                    user_country: users
                        .clone()
                        .read()
                        .await
                        .get(&user_id)
                        .unwrap()
                        .user_country(),
                }
                .into(),
            )
            .into(),
        )
        .await;

    // Handle incoming c2s packets
    while let Some(result) = user_ws_rx.next().await {
        let Ok(msg) = result else {
            eprintln!("websocket error(uid={})", conn_id);
            break;
        };

        let Ok(packetC2S) = PacketC2S::try_from(msg) else {
            eprintln!("invalid packet(uid={})", conn_id);
            continue;
        };

        match packetC2S {
            PacketC2S::SendMessage(msg) => {
                let packet = PacketS2C::SendMessage(MessageS2C {
                    content: msg.content.clone(),
                    user_id: user_id,
                    user_country: users
                        .clone()
                        .read()
                        .await
                        .get(&user_id)
                        .unwrap()
                        .user_country(),
                    message_id: 10,
                });

                connections.send_to_others(packet, conn_id).await;
                insert_frame(msg.content);
            }
        }
    }

    // when the internal channel sends a message, send it out on the websocket.
    // this just directly forwards the message to the websocket.
    tokio::task::spawn(async move {
        while let Some(packetS2C) = internal_rx.next().await {
            let msg = Message::text(serde_json::to_string(&packetS2C).unwrap());

            user_ws_tx
                .send(msg)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // user_ws_rx stream will keep processing as long as the user stays connected.
    // Once they disconnect, the loop exits and the connection is closed.
    connections.close(conn_id).await;
}
