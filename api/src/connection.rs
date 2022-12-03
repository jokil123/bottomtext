use common::packets::s2c::PacketS2C;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::user::User;

// This struct represents a connection
pub struct Connection {
    // mpsc channel used to send messages to the connection
    pub tx: mpsc::UnboundedSender<PacketS2C>,
    pub user: Arc<User>,
}
