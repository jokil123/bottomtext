use std::net::IpAddr;
use std::str::FromStr;
use std::time::Instant;

use crate::util::ip_hash;

pub struct User {
    // TODO implement cooldown using ip
    pub ip: IpAddr,
    pub last_message: Option<Instant>,
}

impl User {
    pub fn id(&self) -> u128 {
        ip_hash(&self.ip)
    }

    pub fn user_country(&self) -> isocountry::CountryCode {
        // TODO implement user country
        isocountry::CountryCode::for_alpha2("BR").unwrap()
    }
}
