use crate::connection::handle_connection;
use std::collections::VecDeque;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start_server(tcp_connections: &Arc<Mutex<VecDeque<Box<TcpStream>>>>) {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => match tcp_connections.lock() {
                Ok(mut queue) => {
                    queue.push_back(Box::new(tcp_stream));
                }
                Err(_) => {}
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

// Didn't use an Async runtime because I wanted to keep dependencies to a minimum
pub fn spawn_thread_pool(
    file_dir: &'static str,
    tcp_connections: &Arc<Mutex<VecDeque<Box<TcpStream>>>>,
) {
    for _ in 0..thread::available_parallelism().unwrap().get() {
        let tcp_connections_clone = Arc::clone(&tcp_connections);

        thread::spawn(move || {
            loop {
                match tcp_connections_clone.lock() {
                    Ok(mut queue) => {
                        if let Some(mut connection) = queue.pop_front() {
                            // Spawn only 1 thread per TCP connection and keep handling requests
                            // with the same connection
                            thread::spawn(move || {
                                // Check if connection is still open
                                while connection.peek(&mut []).is_ok() {
                                    handle_connection(&mut connection, &file_dir);
                                }
                            });
                        }
                    }
                    Err(_) => {}
                }
            }
        });
    }
}
