use std::{
    string::ToString,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
};
use error::*;

pub(crate) enum TargetHost {
    IPv4(String),
    IPv6(String),
    Hostname(String),
}

pub struct Target {
    pub(crate) host: TargetHost,
    pub(crate) port: u16,
}

impl Target {
    pub(crate) fn new(host: TargetHost, port: u16) -> Self {
        Self {
            host,
            port,
        }
    }
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match &self.host {
            TargetHost::IPv4(host) => format!("{}:{}", host, self.port),
            TargetHost::IPv6(host) => format!("{}:{}", host, self.port),
            TargetHost::Hostname(host) => format!("{}:{}", host, self.port),
        }
    }
}

pub trait ToTarget {
    fn to_target(&self) -> Result<Target, MalformedTargetError>;
}

impl ToTarget for &str {
    fn to_target(&self) -> Result<Target, MalformedTargetError> {
        let colon_pos = match self.find(':') {
            Some(pos) => pos,
            None => return Err(MalformedTargetError),
        };

        let host = &self[0..colon_pos];
        let port_str = &self[colon_pos + 1..];

        let port = match port_str.parse::<u16>() {
            Ok(port) => port,
            Err(_) => return Err(MalformedTargetError),
        };

        let host = if host.starts_with('[') && host.ends_with(']') {
            match host[1..host.len() - 1].parse::<Ipv6Addr>() {
                Ok(_) => TargetHost::IPv6(host.into()),
                Err(_) => return Err(MalformedTargetError),
            }
        } else {
            match host.parse::<Ipv4Addr>() {
                Ok(_) => TargetHost::IPv4(host.into()),
                Err(_) => {
                    if host.is_empty() || host.ends_with('.') {
                        return Err(MalformedTargetError);
                    }

                    if host.chars().any(|c| !c.is_ascii_alphanumeric() && c != '.' && c != '-') {
                        return Err(MalformedTargetError);
                    }

                    TargetHost::Hostname(host.into())
                },
            }
        };

        Ok(Target::new(host, port))
    }
}

impl ToTarget for String {
    fn to_target(&self) -> Result<Target, MalformedTargetError> {
        (&**self).to_target()
    }
}

impl ToTarget for SocketAddr {
    fn to_target(&self) -> Result<Target, MalformedTargetError> {
        let host = match self {
            SocketAddr::V4(ip) => TargetHost::IPv4(format!("{}", ip)),
            SocketAddr::V6(ip) => TargetHost::IPv4(format!("[{}]", ip)),
        };
        let port = self.port();

        Ok(Target::new(host, port))
    }
}