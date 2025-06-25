use crate::http::content_type::ContentType;
use crate::http::header::Header;
use crate::http::http_status::HttpStatus;
use std::collections::HashMap;
use std::io::Write;
use std::net::{Shutdown, TcpStream};

pub struct Response<'a> {
    pub status: HttpStatus,
    pub headers: HashMap<String, String>,
    pub body: &'a [u8],
    pub should_close_connection: bool,
}

impl<'a> Response<'a> {
    pub fn new(
        status: HttpStatus,
        content_type: &'a ContentType,
        body: &'a [u8],
        should_close_connection: bool,
    ) -> Response<'a> {
        let mut headers = HashMap::new();
        headers.insert(Header::ContentType.as_string(), content_type.as_string());
        headers.insert(Header::ContentLength.as_string(), body.len().to_string());

        Response {
            status,
            headers,
            body,
            should_close_connection,
        }
    }

    pub fn write(&self, tcp_stream: &mut TcpStream) {
        let mut metadata = String::with_capacity(128);
        metadata.push_str(self.status.as_string());

        for (header_name, header_value) in &self.headers {
            metadata.push_str(&format!("{header_name}: {header_value}\r\n"))
        }

        if self.should_close_connection
            && !self.headers.contains_key(&Header::Connection.as_string())
        {
            metadata.push_str(&format!(
                "{}: {}\r\n",
                Header::Connection.as_string(),
                String::from("close")
            ));
        }

        metadata.push_str("\r\n");

        tcp_stream.write(metadata.as_bytes()).unwrap();
        tcp_stream.write(self.body).unwrap();

        if self.should_close_connection {
            tcp_stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}
