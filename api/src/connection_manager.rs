use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;

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
        tx: mpsc::UnboundedSender<Message>,
        ip: IpAddr,
    ) -> usize {
        // Aquire a write lock on the active connections
        let mut users_lock = users.write().await;

        // Check if the user is already in the users list, if not add them
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
}
