mod http;
mod socks5;

use core::task::{Context, Poll};
use std::{
    result,
    pin::Pin,
    io::{Error, IoSlice},
};
use tokio::{
    net::TcpStream,
    io::{AsyncRead, AsyncWrite, ReadBuf},
};
use async_trait::async_trait;
use crate::{
    prelude::*,
    target::ToTarget,
};

#[async_trait]
pub trait AsyncProxy {
    async fn connect(&self, addr: impl ToTarget + Send) -> Result<ProxyTcpStream>;
}

pub struct ProxyTcpStream {
    stream: TcpStream,
}

impl ProxyTcpStream {
    pub fn into_tcpstream(self) -> TcpStream {
        self.stream
    }
}

impl AsyncRead for ProxyTcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>
    ) -> Poll<result::Result<(), Error>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for ProxyTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<result::Result<usize, Error>> {
        Pin::new(&mut self.stream).poll_write(cx, buf)
    }

    fn poll_write_vectored(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>]
    ) -> Poll<result::Result<usize, Error>> {
        Pin::new(&mut self.stream).poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.stream.is_write_vectored()
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<result::Result<(), Error>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<result::Result<(), Error>> {
        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}