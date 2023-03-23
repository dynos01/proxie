pub mod proxy;
mod error;
mod target;

#[cfg(feature = "enable_sync")]
pub mod sync;

#[cfg(feature = "enable_async_std")]
pub mod async_std;

#[cfg(feature = "enable_tokio")]
pub mod tokio;
mod utils;
mod prelude;

pub use proxy::{Auth, Proxy, HTTPProxy, SOCKS5Proxy};