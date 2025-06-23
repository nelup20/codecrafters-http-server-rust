mod status;
mod request;
mod header;
mod response;
mod content_type;

use crate::request::handle_connection;
use std::net::TcpListener;
use std::{env, thread};
use std::sync::Arc;

fn main() {
    let mut args = env::args();
    let mut dir_arg = String::new();
    
    for arg in args.by_ref() {
        if arg == "--directory" {
            dir_arg = args.next().unwrap();
            break
        }
    }
    
    let base_dir = Arc::new(dir_arg);
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
         match stream {
             Ok(mut tcp_stream) => {
                 let file_dir = Arc::clone(&base_dir);
                 thread::spawn(move || handle_connection(&mut tcp_stream, &file_dir));
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
