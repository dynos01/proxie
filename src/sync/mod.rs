mod http;

use std::{
    error::Error,
    result,
};
use crate::{
    proxy::Proxy,
    target::Target,
};

type Result<T> = result::Result<T, Box<dyn Error>>;

struct ProxiedTcpStream {}

impl ProxiedTcpStream {
    pub fn connect(addr: Target, proxy: Proxy) -> Result<Self> {

        Ok(ProxiedTcpStream {
        })
    }
}