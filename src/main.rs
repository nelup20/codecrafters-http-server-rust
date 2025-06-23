use std::io::{Read, Write};
use std::net::TcpListener;

const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

const SUCCESS_STATUS_LINE: &str = "HTTP/1.1 200 OK\r\n";
const CONTENT_TYPE_HEADER: &str = "Content-Type: text/plain\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
         match stream {
             Ok(mut tcp_stream) => {
                 let mut request_buffer = [0u8; 256];
                 tcp_stream.read(&mut request_buffer).expect("Error reading into request buffer.");

                 let request = String::from_utf8(request_buffer.to_vec()).expect("Error converting request buffer to UTF-8");

                 match parse_request_line(&request) {
                     Some((_, target, _)) => {
                         match target {
                             "/" => {
                                 tcp_stream.write(format!("{}\r\n", SUCCESS_STATUS_LINE).as_bytes()).expect("Error writing 200 OK response to TCP stream.");
                             },
                             to_echo if target.starts_with("/echo/") => {
                                 let (_, body) = to_echo.split_once("/echo/").unwrap();
                                 
                                 let mut response = String::with_capacity(128);
                                 response.push_str(SUCCESS_STATUS_LINE);
                                 response.push_str(CONTENT_TYPE_HEADER);
                                 response.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
                                 response.push_str(body);
                                 
                                 tcp_stream.write(response.as_bytes()).unwrap();
                             },
                             _ => {
                                 tcp_stream.write(NOT_FOUND_RESPONSE.as_bytes()).expect("Error writing 404 Not Found response to TCP stream.");
                             }
                         }
                     },
                     None => {}
                 }
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}

fn parse_request_line(request: &str) -> Option<(&str, &str, &str)> {
    let (request_line, _) = request.split_once("\r\n")?;
    let [http_method, target, http_version] = request_line.split_whitespace().collect::<Vec<&str>>()[..] else { return None };
    Some((http_method, target, http_version))
}