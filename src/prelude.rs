use std::{
    error::Error,
    result,
};

pub(crate) type Result<T> = result::Result<T, Box<dyn Error>>;