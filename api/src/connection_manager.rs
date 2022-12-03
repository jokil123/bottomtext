use common::packets::s2c::PacketS2C;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::connection::Connection;
use crate::user::User;
use crate::users::Users;

// This struct is used to keep track of all the connections
pub struct ConnectionManager {
    next_conn_id: AtomicUsize,
    pub active_connections: Arc<RwLock<HashMap<usize, Connection>>>,
}

impl ConnectionManager {
    // Create a new connection manager
    pub fn new() -> Self {
        Self {
            next_conn_id: AtomicUsize::new(0),
            active_connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Open a new connection and add it to the active connections
    pub async fn open(
        &self,
        users: Users,
        tx: mpsc::UnboundedSender<PacketS2C>,
        ip: IpAddr,
        id: u128,
    ) -> usize {
        // Aquire a write lock on the active connections
        let mut users_lock = users.write().await;

        // Check if the user is already in the users list, if not add them
        let user = match users_lock.get(&id) {
            Some(user) => user,
            None => {
                let user = User {
                    ip,
                    last_message: None,
                };
                users_lock.insert(id, Arc::new(user));
                users_lock.get(&id).unwrap()
            }
        };

        // Get the next connection id
        let conn_id = self.next_conn_id.fetch_add(1, Ordering::Relaxed);

        // Add the connection to the active connections
        self.active_connections.write().await.insert(
            conn_id,
            Connection {
                tx,
                user: user.clone(),
            },
        );

        // Return the connection id
        conn_id
    }

    // Close a connection and remove it from the active connections
    pub async fn close(&self, conn_id: usize) {
        self.active_connections.write().await.remove(&conn_id);
    }

    pub async fn send_to_all(&self, packet: PacketS2C) {
        let active_connections = self.active_connections.read().await;
        for (_, connection) in active_connections.iter() {
            connection.tx.send(packet.clone());
        }
    }

    pub async fn send_to_others(&self, packet: PacketS2C, conn_id: usize) {
        let active_connections = self.active_connections.read().await;
        for (id, connection) in active_connections.iter() {
            if *id != conn_id {
                connection.tx.send(packet.clone());
            }
        }
    }
}
