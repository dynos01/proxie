mod http;

use std::{
    net::{TcpStream},
};
use crate::{
    prelude::*,
    target::ToTarget,
};

pub trait SyncProxy {
    fn connect(&self, addr: impl ToTarget) -> Result<TcpStream>;
}