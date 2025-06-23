use std::io::{Read, Write};
use std::net::TcpListener;

const SUCCESS_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

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
                        if target == "/" {
                            tcp_stream.write(SUCCESS_RESPONSE.as_bytes()).expect("Error writing 200 OK response to TCP stream.");
                        } else {
                            tcp_stream.write(NOT_FOUND_RESPONSE.as_bytes()).expect("Error writing 404 Not Found response to TCP stream.");
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