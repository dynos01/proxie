use std::{
    net::{TcpStream},
    io::{BufRead, Write, BufReader},
};
use crate::{
    prelude::*,
    utils::*,
    target::ToTarget,
    proxy::HTTPProxy,
    sync::{SyncProxy, SyncTcpStream},
};

impl SyncProxy for HTTPProxy {
    fn connect(&self, addr: impl ToTarget) -> Result<SyncTcpStream> {
        let request = make_http_connect_request(&addr, &self)?;

        let mut stream = TcpStream::connect((&*self.server, self.port))?;
        stream.write_all(request.as_bytes())?;
        stream.flush()?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();

        loop {
            reader.read_line(&mut buffer)?;

            if buffer.ends_with("\r\n\r\n") {
                break;
            }
        }

        parse_http_response(&buffer.as_bytes())?;

        Ok(SyncTcpStream {
            stream,
        })
    }
}