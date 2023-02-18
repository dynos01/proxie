use std::{
    net::{SocketAddr, IpAddr}
};

struct Auth {
    username: String,
    password: String,
}

impl Auth {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
        }
    }
}

struct HTTPConfig {
    server: SocketAddr,
    auth: Option<Auth>,
}

impl HTTPConfig {
    pub fn new(ip: IpAddr, port: u16, auth: Option<Auth>) -> Self {
        Self {
            server: SocketAddr::new(ip, port),
            auth: auth,
        }
    }
}

pub enum Proxy {
    Direct,
    HTTP(HTTPConfig),
}