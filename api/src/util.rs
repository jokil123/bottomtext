use std::net::IpAddr;

use common::db::legacy_db::insert_frame;
use common::packets::MessageContent;
use sha2::Digest;
use sha2::Sha256;

pub fn ip_hash(ip: &IpAddr) -> u128 {
    // TODO fix this
    let mut hasher = Sha256::new();
    hasher.update(ip.to_string());
    let hash = hasher.finalize().to_vec();
    hash.split_at(16)
        .0
        .iter()
        .fold(0, |acc, x| acc * 256 + *x as u128)
}
