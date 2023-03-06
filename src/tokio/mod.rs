mod http;
mod socks5;

use tokio::net::TcpStream;
use async_trait::async_trait;
use crate::{
    prelude::*,
    target::ToTarget,
};

#[async_trait]
pub trait AsyncProxy {
    async fn connect(&self, addr: impl ToTarget + Send) -> Result<TcpStream>;
}