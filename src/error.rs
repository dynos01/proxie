use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    net::SocketAddr,
};

#[derive(Debug)]
pub struct MalformedTargetError;

impl Display for MalformedTargetError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Malformed target address. Correct ones should be like: \"example.com:80\", \"1.2.3.4:443\", or \"[2001:db8::1]:443\". ")
    }
}

impl Error for MalformedTargetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct HTTPNotOKError {
    pub(crate) code: Option<u16>,
}

impl Display for HTTPNotOKError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.code {
            Some(code) => write!(f, "HTTP proxy server responsed with a non-200 error code: {}. ", code),
            None => write!(f, "HTTP proxy server response invalid. "),
        }
    }
}

impl Error for HTTPNotOKError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub(crate) struct FailedToConnectError {
    pub(crate) socket: SocketAddr,
}

impl Display for FailedToConnectError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed to connect to remote host: {}. ", self.socket)
    }
}

impl Error for FailedToConnectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}