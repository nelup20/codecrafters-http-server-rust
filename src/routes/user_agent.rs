use std::net::TcpStream;
use crate::content_type::ContentType;
use crate::request::{should_close_connection, LINE_BREAK};
use crate::response::write_response;
use crate::status::Status;

pub fn handle_get_user_agent(tcp_stream: &mut TcpStream, request: &str) {
    for line in request.split(LINE_BREAK) {
        if line.to_lowercase().starts_with("user-agent") {
            let user_agent = line.split_once(": ").unwrap().1.replace(LINE_BREAK, "");

            write_response(
                tcp_stream,
                Status::Ok,
                ContentType::TextPlain,
                None,
                user_agent.as_bytes(),
                should_close_connection(request),
            );
            return;
        }
    }
}