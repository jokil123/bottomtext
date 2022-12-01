use std::{env, net::SocketAddr};

// Read the IP address from the environment variable
pub fn get_ip() -> SocketAddr {
    let address = env::var("API_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("API_PORT").unwrap_or("3030".to_string());

    format!("{}:{}", address, port)
        .parse::<SocketAddr>()
        .unwrap()
}
