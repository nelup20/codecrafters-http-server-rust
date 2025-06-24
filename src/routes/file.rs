use std::fs;
use std::net::TcpStream;
use crate::content_type::ContentType;
use crate::request::{should_close_connection, LINE_BREAK};
use crate::response::write_response;
use crate::routes::not_found::handle_not_found;
use crate::status::Status;

pub fn handle_get_file(tcp_stream: &mut TcpStream, request: &str, request_path: &str, file_dir: &str) {
    let (_, file) = request_path.split_once("/files/").unwrap();

    let local_file = fs::read_dir(file_dir)
        .unwrap()
        .map(|entry| entry.unwrap())
        .find(|entry| entry.file_name() == file && entry.metadata().unwrap().is_file());

    match local_file {
        Some(found_file) => write_response(
            tcp_stream,
            Status::Ok,
            ContentType::OctetStream,
            None,
            &fs::read_to_string(found_file.path()).unwrap().as_bytes(),
            should_close_connection(request),
        ),
        None => handle_not_found(tcp_stream, request),
    }
}

pub fn handle_post_file(tcp_stream: &mut TcpStream, request: &str, request_path: &str, file_dir: &str) {
    let (_, file_name) = request_path.split_once("/files/").unwrap();
    let (metadata, body) = request.split_once("\r\n\r\n").unwrap();

    for header in metadata.split(LINE_BREAK) {
        if header.starts_with("Content-Length") {
            let parsed_length: usize = header.split_once(": ").unwrap().1.parse().unwrap();
            fs::write(file_dir.to_owned() + file_name, &body[..parsed_length]).unwrap();
            write_response(
                tcp_stream,
                Status::Created,
                ContentType::TextPlain,
                None,
                &[],
                should_close_connection(request),
            );
        }
    }
}