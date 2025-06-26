use crate::http::request::Request;
use crate::routes::echo::handle_get_echo;
use crate::routes::file::{handle_get_file, handle_post_file};
use crate::routes::not_found::handle_not_found;
use crate::routes::root::handle_get_root;
use crate::routes::user_agent::handle_get_user_agent;
use std::io::Read;
use std::net::TcpStream;

pub fn handle_connection(tcp_stream: &mut TcpStream, file_dir: &str) {
    let mut request_buffer = [0u8; 2048];
    tcp_stream
        .read(&mut request_buffer)
        .expect("Error reading into request buffer.");

    if request_buffer[0] == 0 {
        return;
    }

    let request_string = String::from_utf8(request_buffer.to_vec())
        .expect("Error converting request buffer to UTF-8");

    let request = Request::from_string(&request_string);

    match request.path {
        "/" => handle_get_root(tcp_stream, &request),
        "/user-agent" => handle_get_user_agent(tcp_stream, &request),
        request_path if request_path.starts_with("/echo/") => handle_get_echo(tcp_stream, &request),
        request_path if request_path.starts_with("/files/") => match request.method {
            "GET" => handle_get_file(tcp_stream, &request, file_dir),
            "POST" => handle_post_file(tcp_stream, &request, file_dir),
            _ => {}
        },
        _ => handle_not_found(tcp_stream, &request),
    }
}
