use crate::routes::echo::handle_get_echo;
use crate::routes::file::{handle_get_file, handle_post_file};
use crate::routes::not_found::handle_not_found;
use crate::routes::root::handle_get_root;
use crate::routes::user_agent::handle_get_user_agent;
use std::io::Read;
use std::net::TcpStream;

pub const LINE_BREAK: &'static str = "\r\n";

pub fn handle_connection(tcp_stream: &mut TcpStream, file_dir: &str) {
    let mut request_buffer = [0u8; 256];
    tcp_stream
        .read(&mut request_buffer)
        .expect("Error reading into request buffer.");

    let request = String::from_utf8(request_buffer.to_vec())
        .expect("Error converting request buffer to UTF-8");

    match parse_request_line(&request) {
        Some((method, target, _)) => match target {
            "/" => handle_get_root(tcp_stream, &request),
            "/user-agent" => handle_get_user_agent(tcp_stream, &request),
            request_path if target.starts_with("/echo/") => handle_get_echo(tcp_stream, &request, &request_path),
            request_path if target.starts_with("/files/") => {
                match method {
                    "GET" => handle_get_file(tcp_stream, &request, &request_path, file_dir),
                    "POST" => handle_post_file(tcp_stream, &request, &request_path, file_dir),
                    _ => {}
                }
                
            }
            _ => handle_not_found(tcp_stream, &request),
        },
        None => {}
    }
}

fn parse_request_line(request: &str) -> Option<(&str, &str, &str)> {
    let (request_line, _) = request.split_once(LINE_BREAK)?;
    let [http_method, target, http_version] =
        request_line.split_whitespace().collect::<Vec<&str>>()[..]
    else {
        return None;
    };
    Some((http_method, target, http_version))
}

#[inline(always)]
pub fn should_close_connection(request: &str) -> bool {
    request.contains("Connection: close")
}
