mod status;
mod request;
mod header;
mod response;

use crate::request::handle_connection;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
         match stream {
             Ok(tcp_stream) => {
                 thread::spawn(|| handle_connection(tcp_stream));
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
