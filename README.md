# proxie: Rust crate for SOCKS/HTTP proxy client
`proxie` offers easy SOCKS/HTTP client-side proxy integration into your existing socket program, with support for `tokio`, `async-std` and synchronous `std::net` runtime. [docs.rs](https://docs.rs/proxie/latest/proxie/)

[![Crate](https://img.shields.io/crates/v/proxie.svg)](https://crates.io/crates/proxie)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

**Warning: this crate is still under active development. It is POSSIBLE the API might change in the future.**

## Example
`proxie` aims to provide a set of API that corresponds to its runtimes' native API. Here is an example of using `tokio` to make HTTP request via SOCKS5 proxy:

```
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use futures::future;
use proxie::{SOCKS5Proxy, tokio::AsyncProxy, Auth};

#[tokio::main]
async fn main() {
    let n_thread = 10;
    let mut handles = vec![];

    for _ in 0..n_thread {
        handles.push(tokio::spawn(async {
            let server = SOCKS5Proxy::new("127.0.0.1", 1080, None);

            let mut stream = server.connect("example.com:80").await.unwrap();
            //let mut stream = TcpStream::connect("example.com:80").await.unwrap();

            stream.write_all(b"GET / HTTP/1.1\r\n\r\n").await.unwrap();
            stream.flush().await.unwrap();

            let mut reader = BufReader::new(&mut stream);
            let mut buffer = String::new();

            loop {
                reader.read_line(&mut buffer).await.unwrap();

                if buffer.ends_with("\r\n\r\n") {
                    print!("{}", buffer);
                    break;
                }
            }
        }));
    }

    future::join_all(handles).await;
}
```

This crates replaces the `TcpStream` creation process with two lines. No other modifications are needed.

To use username and password authentication, simply import `proxie::Auth` and use this line to create a proxy server instance:

```
let server = SOCKS5Proxy::new("127.0.0.1", 1080, Auth::new("user", "pass"));
```

To enable support for each runtime, add corresponding feature into your `Cargo.toml`:

```
proxie = { version = "0.1.0", features = ["enable_tokio"] }
# Features currently available are: "enable_sync", "enable_tokio", and "enable_async_std".
```

## Features and to-do
Runtime support:
- [x] synchronous `std::net`
- [x] `tokio`
- [x] `async-std`

Protocol support:
- [x] HTTP CONNECT
- [x] SOCKS5 CONNECT
- [ ] SOCKS5 BIND
- [ ] SOCKS5 UDP ASSOCIATE
- [ ] SOCKS4a
- [ ] HTTPS CONNECT
- [ ] Chained proxy

Other works:
- Documentation
- Full implementation of `TcpStream` methods
- Unit tests

## Development
Basic tests on current features are done, however it is not guaranteed that everything will work. It is also not guaranteed that current API will remain the same forever, though I will do my best to keep it stable. For bug report, feature request and any other suggestions please use GitHub issue or contact me directly.

## License
MIT