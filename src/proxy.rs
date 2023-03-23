#[derive(Clone)]
pub struct Auth {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Auth {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
        }
    }
}

#[derive(Clone)]
pub enum Proxy {
    HTTP(HTTPProxy),
    SOCKS5(SOCKS5Proxy),
}

#[derive(Clone)]
pub struct HTTPProxy {
    pub(crate) server: String,
    pub(crate) port: u16,
    pub(crate) auth: Option<Auth>,
}

impl HTTPProxy {
    pub fn new<T: Into<Option<Auth>>>(server: &str, port: u16, auth: T) -> Self {
        let auth = auth.into();
        Self {
            server: server.into(),
            port,
            auth,
        }
    }
}

#[derive(Clone)]
pub struct SOCKS5Proxy {
    pub(crate) server: String,
    pub(crate) port: u16,
    pub(crate) auth: Option<Auth>,
}

impl SOCKS5Proxy {
    pub fn new<T: Into<Option<Auth>>>(server: &str, port: u16, auth: T) -> Self {
        let auth = auth.into();
        Self {
            server: server.into(),
            port,
            auth,
        }
    }
}

pub(crate) enum SOCKS5Command {
    CONNECT,
}
