use crate::compression_scheme::CompressionScheme;
use crate::content_type::ContentType;
use crate::header::Header;
use crate::status::Status;
use std::io::Write;
use std::net::{Shutdown, TcpStream};

// Would be cleaner to work with response structs and more Options
pub fn write_response(
    tcp_stream: &mut TcpStream,
    status: Status,
    content_type: ContentType,
    content_encoding: Option<CompressionScheme>,
    body: &[u8],
    close_connection: bool,
) {
    let mut metadata = String::with_capacity(128);
    metadata.push_str(status.as_string());
    metadata.push_str(&Header::ContentType(content_type).as_string());
    metadata.push_str(&Header::ContentLength(body.len()).as_string());

    match content_encoding {
        None => {}
        Some(scheme) => metadata.push_str(&Header::ContentEncoding(scheme).as_string()),
    }

    if close_connection {
        metadata.push_str("Connection: close\r\n");
    }

    metadata.push_str("\r\n");

    tcp_stream.write(metadata.as_bytes()).unwrap();
    tcp_stream.write(body).unwrap();

    if close_connection {
        tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
}
