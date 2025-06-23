mod status;
mod request;
mod header;
mod response;

use crate::request::parse_request_line;
use crate::response::write_response;
use crate::status::Status;
use std::io::Read;
use std::net::TcpListener;

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
                                 write_response(tcp_stream, Status::Ok, "");
                             },

                             "/user-agent" => {
                                 for line in request.split("\r\n") {
                                     if line.to_lowercase().starts_with("user-agent") {
                                         let user_agent = line.split_once(": ").unwrap().1.replace("\r\n", "");

                                         write_response(tcp_stream, Status::Ok, &user_agent);
                                         return
                                     }
                                 }
                             },

                             to_echo if target.starts_with("/echo/") => {
                                 let (_, body) = to_echo.split_once("/echo/").unwrap();

                                 write_response(tcp_stream, Status::Ok, &body)
                             },

                             _ => {
                                 write_response(tcp_stream, Status::NotFound, "");
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
