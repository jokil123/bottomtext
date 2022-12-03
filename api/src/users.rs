use std::collections::HashMap;
// use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::user::User;

// This struct keeps track of all users
pub type Users = Arc<RwLock<HashMap<u128, Arc<User>>>>;
