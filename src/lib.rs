pub mod proxy;
mod error;
mod target;
pub mod sync;
mod utils;
mod prelude;

extern crate base64;
extern crate httparse;

pub use proxy::{Auth, HTTPProxy};