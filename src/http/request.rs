use crate::http::compression_scheme::CompressionScheme;
use std::collections::HashMap;
use crate::http::header::Header;

pub const LINE_BREAK: &'static str = "\r\n";
pub const BODY_SEPARATOR: &'static str = "\r\n\r\n";

// Should use enums for method, protocol and header keys, but I'm done refactoring this challenge.
pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub protocol: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub compression_scheme: Option<CompressionScheme>,
    pub body: &'a [u8],
    pub should_close_connection: bool,
}

impl<'a> Request<'a> {
    pub fn from_string(request: &'a str) -> Self {
        let (metadata, body) = request.split_once(BODY_SEPARATOR).unwrap();
        let (request_line, header_line) = metadata.split_once(LINE_BREAK).unwrap();
        let [method, path, protocol] = request_line.split_whitespace().collect::<Vec<&str>>()[..]
        else {
            panic!("Malformed request line")
        };

        let mut headers = HashMap::new();
        for header in header_line.split(LINE_BREAK) {
            let (header_name, header_value) = header.split_once(": ").unwrap();
            headers.insert(header_name, header_value);
        }

        let mut compression_scheme = None;
        if let Some(schemes) = headers.get(Header::AcceptEncoding.as_str()) {
            if schemes
                .split(",")
                .find(|&scheme| scheme.trim() == CompressionScheme::Gzip.as_str())
                .is_some()
            {
                compression_scheme = Some(CompressionScheme::Gzip);
            }
        }

        let mut should_close_connection = false;
        if let Some(&connection_header_value) = headers.get(Header::Connection.as_str()) {
            if connection_header_value == "close" {
                should_close_connection = true;
            }
        }

        Request {
            method,
            path,
            protocol,
            compression_scheme,
            should_close_connection,
            headers,
            body: body.as_bytes(),
        }
    }
}
