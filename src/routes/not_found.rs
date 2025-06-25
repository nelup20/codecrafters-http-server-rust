use crate::http::request::Request;
use crate::http::response::Response;
use std::net::TcpStream;
use crate::http::content_type::ContentType::TextPlain;
use crate::http::http_status::HttpStatus;

pub fn handle_not_found(tcp_stream: &mut TcpStream, request: &Request) {
    let response = Response::new(
        HttpStatus::NotFound,
        &TextPlain,
        &[],
        request.should_close_connection,
    );
    
    response.write(tcp_stream);
}
