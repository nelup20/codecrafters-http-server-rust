mod status;
mod request;
mod header;
mod response;
mod content_type;
mod compression_scheme;
mod routes;

use crate::request::handle_connection;
use std::net::TcpListener;
use std::{env, thread};
use std::sync::{Arc, Mutex};

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
             Ok(tcp_stream) => {
                 let shared_file_dir = Arc::clone(&base_dir);
                 
                 thread::spawn(move || {
                     let shared_connection = Arc::new(Mutex::new(tcp_stream));
                     
                     loop {
                         let file_dir = Arc::clone(&shared_file_dir);
                         let connection = Arc::clone(&shared_connection);

                         thread::spawn(move || {
                             handle_connection(&mut connection.lock().unwrap(), &file_dir);
                         });
                     } 
                 });
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
