use std::io::Read;
use std::net::TcpStream;
use crate::response::write_response;
use crate::status::Status;

pub fn handle_connection(mut tcp_stream: TcpStream) {
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

pub fn parse_request_line(request: &str) -> Option<(&str, &str, &str)> {
    let (request_line, _) = request.split_once("\r\n")?;
    let [http_method, target, http_version] = request_line.split_whitespace().collect::<Vec<&str>>()[..] else { return None };
    Some((http_method, target, http_version))
}