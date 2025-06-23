use crate::status::Status;
use std::io::Write;
use std::net::TcpStream;
use crate::header::Header;

pub fn write_response(mut tcp_stream: TcpStream, status: Status, body: &str) {
    let mut response = String::with_capacity(128);
    response.push_str(status.as_string());
    response.push_str(&Header::ContentType.as_string());
    response.push_str(&Header::ContentLength(body.len()).as_string());
    response.push_str("\r\n");
    response.push_str(body);

    tcp_stream.write(response.as_bytes()).unwrap();
}