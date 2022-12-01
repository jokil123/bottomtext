use common::{db::legacy_db::insert_frame, frame::FrameJson};
use warp::ws::Message;

use crate::connection_manager::ConnectionManager;

pub async fn write_frame_to_db(msg: Message) {
    let s = match msg.to_str() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error converting message to string");
            return;
        }
    };

    // println!("message: {}", s);

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

pub async fn user_frame(conn_id: usize, msg: Message, conn_manager: &ConnectionManager) {
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
