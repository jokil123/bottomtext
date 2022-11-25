use std::sync::Arc;
use tokio::sync::mpsc;
use warp::ws::Message;

use crate::user::User;

// This struct represents a connection
pub struct Connection {
    // mpsc channel used to send messages to the connection
    pub tx: mpsc::UnboundedSender<Message>,
    pub user: Arc<User>,
}
