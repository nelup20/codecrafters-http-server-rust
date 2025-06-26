mod connection;
mod http;
mod routes;
mod server;

use crate::server::{spawn_thread_pool, start_server};
use std::collections::VecDeque;
use std::env;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

fn main() {
    let file_dir = parse_file_dir_from_args();
    let tcp_connections = Arc::new(Mutex::new(VecDeque::<Box<TcpStream>>::new()));

    spawn_thread_pool(&file_dir, &tcp_connections);
    start_server(&tcp_connections);
}

fn parse_file_dir_from_args() -> &'static str {
    let mut args = env::args();
    let mut dir_arg = String::new();

    for arg in args.by_ref() {
        if arg == "--directory" {
            dir_arg = args.next().unwrap();
            break;
        }
    }

    dir_arg.leak()
}
