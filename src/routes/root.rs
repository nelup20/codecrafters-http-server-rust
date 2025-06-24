use std::net::TcpStream;
use crate::content_type::ContentType;
use crate::request::should_close_connection;
use crate::response::write_response;
use crate::status::Status;

pub fn handle_get_root(tcp_stream: &mut TcpStream, request: &str) {
    write_response(
        tcp_stream,
        Status::Ok,
        ContentType::TextPlain,
        None,
        &[],
        should_close_connection(request),
    );
}