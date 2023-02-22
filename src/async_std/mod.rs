mod http;
mod socks5;

use std::{
    result,
    pin::Pin,
};
use async_std::{
    net::TcpStream,
    io::Error,
    task::{Context, Poll},
};
use futures::{AsyncRead, AsyncWrite};
use async_trait::async_trait;
use crate::{
    prelude::*,
    target::ToTarget,
};

#[async_trait]
pub trait AsyncProxy {
    async fn connect(&self, addr: impl ToTarget + Send) -> Result<AsyncTcpStream>;
}

pub struct AsyncTcpStream {
    stream: TcpStream,
}

impl AsyncRead for AsyncTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8]
    ) -> Poll<result::Result<usize, Error>> {
        Pin::new(&mut &self.stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for AsyncTcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<result::Result<usize, Error>> {
        Pin::new(&mut &self.stream).poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<result::Result<(), Error>> {
        Pin::new(&mut &self.stream).poll_flush(cx)
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<result::Result<(), Error>> {
        Pin::new(&mut &self.stream).poll_close(cx)
    }
}

impl AsyncRead for &AsyncTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8]
    ) -> Poll<result::Result<usize, Error>> {
        Pin::new(&mut &self.stream).poll_read(cx, buf)
    }
}

impl AsyncWrite for &AsyncTcpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8]
    ) -> Poll<result::Result<usize, Error>> {
        Pin::new(&mut &self.stream).poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<result::Result<(), Error>> {
        Pin::new(&mut &self.stream).poll_flush(cx)
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<result::Result<(), Error>> {
        Pin::new(&mut &self.stream).poll_close(cx)
    }
}