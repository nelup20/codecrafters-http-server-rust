use crate::http::request::Request;
use crate::http::response::Response;
use std::net::TcpStream;
use crate::http::content_type::ContentType;
use crate::http::header::Header;
use crate::http::http_status::HttpStatus;

pub fn handle_get_user_agent(tcp_stream: &mut TcpStream, request: &Request) {
    if let Some(user_agent) = request.headers.get(&Header::UserAgent.as_string()) {
        let response = Response::new(
            HttpStatus::Ok,
            &ContentType::TextPlain,
            user_agent.as_bytes(),
            request.should_close_connection,
        );
        
        response.write(tcp_stream)
    }
}
