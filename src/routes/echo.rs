use std::io::Write;
use std::net::TcpStream;
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::compression_scheme::CompressionScheme;
use crate::content_type::ContentType;
use crate::request::should_close_connection;
use crate::response::write_response;
use crate::status::Status;

pub fn handle_get_echo(tcp_stream: &mut TcpStream, request: &str, request_path: &str) {
    let (_, body) = request_path.split_once("/echo/").unwrap();

    for header in request.split("\r\n") {
        if header.starts_with("Accept-Encoding") {
            let compression_schemes = header.split_once(": ").unwrap().1;

            let supported_scheme = compression_schemes
                .split(",")
                .find(|&scheme| scheme.trim() == "gzip");
            match supported_scheme {
                None => {}
                Some(_) => {
                    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                    encoder.write_all(body.as_bytes()).unwrap();

                    write_response(
                        tcp_stream,
                        Status::Ok,
                        ContentType::TextPlain,
                        Some(CompressionScheme::Gzip),
                        &encoder.finish().unwrap(),
                        request.contains("Connection: close"),
                    );
                    return;
                }
            }
        }
    }

    write_response(
        tcp_stream,
        Status::Ok,
        ContentType::TextPlain,
        None,
        body.as_bytes(),
        should_close_connection(request),
    );
}