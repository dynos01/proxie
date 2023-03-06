pub mod proxy;
mod error;
mod target;
pub mod sync;
pub mod async_std;
pub mod tokio;
mod utils;
mod prelude;

pub use proxy::{Auth, HTTPProxy, SOCKS5Proxy};