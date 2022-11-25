use std::net::IpAddr;
use std::time::Instant;

pub struct User {
    // TODO implement cooldown using ip
    pub ip: IpAddr,
    pub cooldown_until: Option<Instant>,
}
