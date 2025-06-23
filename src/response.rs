use crate::status::Status;
use std::io::Write;
use std::net::TcpStream;
use crate::compression_scheme::CompressionScheme;
use crate::content_type::ContentType;
use crate::header::Header;

// Would be cleaner to work with response structs and more Options 
pub fn write_response(tcp_stream: &mut TcpStream, status: Status, content_type: ContentType, content_encoding: Option<CompressionScheme>, body: &str) {
    let mut response = String::with_capacity(128);
    response.push_str(status.as_string());
    response.push_str(&Header::ContentType(content_type).as_string());
    response.push_str(&Header::ContentLength(body.len()).as_string());
    
    match content_encoding {
        None => {}
        Some(scheme) => response.push_str(&Header::ContentEncoding(scheme).as_string())
    }
    
    response.push_str("\r\n");
    response.push_str(body);

    tcp_stream.write(response.as_bytes()).unwrap();
}