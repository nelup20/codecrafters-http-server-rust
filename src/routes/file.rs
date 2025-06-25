use crate::http::request::Request;
use crate::http::response::Response;
use crate::routes::not_found::handle_not_found;
use std::fs;
use std::net::TcpStream;
use crate::http::content_type::ContentType;
use crate::http::header::Header;
use crate::http::http_status::HttpStatus;

pub fn handle_get_file(tcp_stream: &mut TcpStream, request: &Request, file_dir: &str) {
    let (_, file) = request.path.split_once("/files/").unwrap();

    let local_file = fs::read_dir(file_dir)
        .unwrap()
        .map(|entry| entry.unwrap())
        .find(|entry| entry.file_name() == file && entry.metadata().unwrap().is_file());

    match local_file {
        Some(found_file) => {
            let file_contents = fs::read_to_string(found_file.path()).unwrap();
            let response_body = &file_contents.as_bytes();

            let response = Response::new(
                HttpStatus::Ok,
                &ContentType::OctetStream,
                response_body,
                request.should_close_connection,
            );

            response.write(tcp_stream);
        }
        None => handle_not_found(tcp_stream, request),
    }
}

pub fn handle_post_file(tcp_stream: &mut TcpStream, request: &Request, file_dir: &str) {
    let (_, file_name) = request.path.split_once("/files/").unwrap();

    if let Some(length) = request.headers.get(Header::ContentLength.as_str()) {
        let parsed_length: usize = length.parse().unwrap();
        fs::write(
            file_dir.to_owned() + file_name,
            &request.body[..parsed_length],
        )
        .unwrap();

        let response = Response::new(
            HttpStatus::Created,
            &ContentType::TextPlain,
            &[],
            request.should_close_connection,
        );

        response.write(tcp_stream);
    }
}
