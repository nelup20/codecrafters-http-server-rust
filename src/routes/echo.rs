use crate::http::request::Request;
use crate::http::response::Response;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;
use std::net::TcpStream;
use crate::http::compression_scheme::CompressionScheme;
use crate::http::content_type::ContentType::TextPlain;
use crate::http::header::Header;
use crate::http::http_status::HttpStatus;

pub fn handle_get_echo(tcp_stream: &mut TcpStream, request: &Request) {
    let (_, body) = request.path.split_once("/echo/").unwrap();

    if let Some(compression_schemes) = request.headers.get(&Header::AcceptEncoding.as_string()) {
        let supported_scheme = compression_schemes
            .split(",")
            .find(|&scheme| scheme.trim() == "gzip");

        match supported_scheme {
            None => {}
            Some(_) => {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(body.as_bytes()).unwrap();
                let compressed_body = &encoder.finish().unwrap();

                let mut response = Response::new(
                    HttpStatus::Ok,
                    &TextPlain,
                    compressed_body,
                    request.should_close_connection,
                );

                response.headers.insert(
                    Header::ContentEncoding.as_string(),
                    CompressionScheme::Gzip.as_string(),
                );

                response.write(tcp_stream);
                return;
            }
        }
    }

    let response = Response::new(
        HttpStatus::Ok,
        &TextPlain,
        body.as_bytes(),
        request.should_close_connection,
    );

    response.write(tcp_stream);
}
