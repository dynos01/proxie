use async_std::{
    net::TcpStream,
    io::{prelude::*, WriteExt, BufReader},
};
use async_trait::async_trait;
use crate::{
    prelude::*,
    utils::*,
    target::ToTarget,
    proxy::HTTPProxy,
    async_std::{AsyncProxy, ProxyTcpStream},
};

#[async_trait]
impl AsyncProxy for HTTPProxy {
    async fn connect(&self, addr: impl ToTarget + Send) -> Result<ProxyTcpStream> {
        let request = make_http_connect_request(&addr, &self)?;

        let mut stream = TcpStream::connect((&*self.server, self.port)).await?;
        stream.write_all(request.as_bytes()).await?;
        stream.flush().await?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        loop {
            reader.read_line(&mut buffer).await?;

            if buffer.ends_with("\r\n\r\n") {
                break;
            }
        }

        parse_http_response(&buffer.as_bytes())?;

        Ok(ProxyTcpStream {
            stream
        })
    }
}