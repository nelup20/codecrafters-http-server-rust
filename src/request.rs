use crate::content_type::ContentType;
use crate::response::write_response;
use crate::status::Status;
use std::fs;
use std::io::Read;
use std::net::TcpStream;

const LINE_BREAK: &'static str = "\r\n";

pub fn handle_connection(tcp_stream: &mut TcpStream, file_dir: &str) {
    let mut request_buffer = [0u8; 256];
    tcp_stream
        .read(&mut request_buffer)
        .expect("Error reading into request buffer.");

    let request = String::from_utf8(request_buffer.to_vec())
        .expect("Error converting request buffer to UTF-8");

    match parse_request_line(&request) {
        Some((method, target, _)) => match target {
            "/" => handle_root(tcp_stream),
            "/user-agent" => handle_user_agent(tcp_stream, &request),
            path if target.starts_with("/echo/") => handle_echo(tcp_stream, path),
            path if target.starts_with("/files/") && method == "GET" => handle_get_file(tcp_stream, path, file_dir),
            path if target.starts_with("/files/") && method == "POST" => handle_post_file(tcp_stream, path, file_dir, &request),
            _ => handle_not_found(tcp_stream),
        },
        None => {}
    }
}

fn handle_not_found(tcp_stream: &mut TcpStream) {
    write_response(tcp_stream, Status::NotFound, ContentType::TextPlain, "");
}

fn handle_echo(tcp_stream: &mut TcpStream, path: &str) {
    let (_, body) = path.split_once("/echo/").unwrap();

    write_response(tcp_stream, Status::Ok, ContentType::TextPlain, &body);
}

fn handle_get_file(tcp_stream: &mut TcpStream, path: &str, file_dir: &str) {
    let (_, file) = path.split_once("/files/").unwrap();

    let local_file = fs::read_dir(file_dir)
        .unwrap()
        .map(|entry| entry.unwrap())
        .find(|entry| entry.file_name() == file && entry.metadata().unwrap().is_file());

    match local_file {
        Some(found_file) => write_response(
            tcp_stream,
            Status::Ok,
            ContentType::OctetStream,
            &fs::read_to_string(found_file.path()).unwrap(),
        ),
        None => handle_not_found(tcp_stream),
    }
}

fn handle_post_file(tcp_stream: &mut TcpStream, path: &str, file_dir: &str, request: &str) {
    let (_, file_name) = path.split_once("/files/").unwrap();
    let (metadata, body) = request.split_once("\r\n\r\n").unwrap();
    
    for header in metadata.split(LINE_BREAK) {
        if header.starts_with("Content-Length") {
            let parsed_length: usize = header.split_once(": ").unwrap().1.parse().unwrap();
            fs::write(file_dir.to_owned() + file_name, &body[..parsed_length]).unwrap();
            write_response(tcp_stream, Status::Created, ContentType::TextPlain, "");
        }
    }
}

fn handle_root(tcp_stream: &mut TcpStream) {
    write_response(tcp_stream, Status::Ok, ContentType::TextPlain, "");
}

fn handle_user_agent(tcp_stream: &mut TcpStream, request: &str) {
    for line in request.split(LINE_BREAK) {
        if line.to_lowercase().starts_with("user-agent") {
            let user_agent = line.split_once(": ").unwrap().1.replace(LINE_BREAK, "");

            write_response(tcp_stream, Status::Ok, ContentType::TextPlain, &user_agent);
            return;
        }
    }
}

pub fn parse_request_line(request: &str) -> Option<(&str, &str, &str)> {
    let (request_line, _) = request.split_once(LINE_BREAK)?;
    let [http_method, target, http_version] =
        request_line.split_whitespace().collect::<Vec<&str>>()[..]
    else {
        return None;
    };
    Some((http_method, target, http_version))
}
