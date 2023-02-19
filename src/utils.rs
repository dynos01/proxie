use base64::{Engine as _, engine::general_purpose};
use httparse::{Response, EMPTY_HEADER};
use crate::{
    prelude::*,
    error::*,
    target::ToTarget,
    proxy::HTTPProxy,
};

pub(crate) fn make_http_connect_request(addr: &impl ToTarget, proxy: &HTTPProxy) -> Result<String> {
    let addr = addr.to_target()?.to_string();
    let mut request = format!(
        "CONNECT {0} HTTP/1.1\r\n\
        Host: {0}\r\n\
        User-Agent: proxie/0.0\r\n\
        Proxy-Connection: Keep-Alive\r\n",
        addr
    );

    match &proxy.auth {
        Some(auth) => {
            let raw_auth = format!("{}:{}", auth.username, auth.password);
            let encoded_auth: String = general_purpose::STANDARD.encode(raw_auth.as_bytes());
            request.push_str(&format!("Proxy-Authorization: Basic {}\r\n", encoded_auth));
        },
        None => {},
    };

    request.push_str("\r\n");

    Ok(request)
}

pub(crate) fn parse_http_response(buffer: &[u8]) -> Result<()> {
    let mut headers = [EMPTY_HEADER; 64];
    let mut response = Response::new(&mut headers);

    response.parse(&buffer)?;

    match response.code {
        Some(code) => {
            if code != 200 {
                return Err(Box::new(HTTPNotOKError {
                    code: Some(code),
                }));
            }
        },
        None => return Err(Box::new(HTTPNotOKError {
            code: None,
        })),
    };

    Ok(())
}