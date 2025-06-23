use std::io::Write;
use std::net::TcpListener;

const RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";

fn main() {

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
         match stream {
             Ok(mut tcp_stream) => {
                 tcp_stream.write(RESPONSE.as_bytes()).expect("Error writing 200 OK response to TCP stream.");
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
