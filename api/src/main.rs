use api::get_ip::get_ip;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

use api::connection_manager::ConnectionManager;
use api::users::Users;
use api::websocket::user_connected;
use common::db::legacy_db::read_frames;

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

    let static_files_path = env::var("STATIC_PATH").unwrap_or("..ui/dist".to_string());
    let static_files = warp::fs::dir(static_files_path);

    let routes = frames.or(ws).or(static_files);

    println!("Binding to {}", get_ip());
    warp::serve(routes).run(get_ip()).await;

    println!("Server stopped");
}
