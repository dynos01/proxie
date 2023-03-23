mod http;
mod socks5;

use std::{
    net::TcpStream,
    io::{Read, Write, Result as IOResult},
};
use crate::{
    prelude::*,
    target::ToTarget,
};

pub trait SyncProxy {
    fn connect(&self, addr: impl ToTarget) -> Result<ProxyTcpStream>;
}

pub struct ProxyTcpStream {
    stream: TcpStream,
}

impl ProxyTcpStream {
    pub fn into_tcpstream(self) -> TcpStream {
        self.stream
    }
}

impl Read for ProxyTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        self.stream.read(buf)
    }
}

impl Write for ProxyTcpStream {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> IOResult<()> {
        self.stream.flush()
    }
}

impl Read for &ProxyTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        (&self.stream).read(buf)
    }
}

impl Write for &ProxyTcpStream {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        (&self.stream).write(buf)
    }

    fn flush(&mut self) -> IOResult<()> {
        (&self.stream).flush()
    }
}