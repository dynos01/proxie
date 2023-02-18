use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
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