use crate::http::content_type::ContentType;
use crate::http::header::Header;
use crate::http::http_status::HttpStatus;
use std::collections::HashMap;
use std::io::Write;
use std::net::{Shutdown, TcpStream};

pub struct Response<'a> {
    pub status: HttpStatus,
    pub headers: HashMap<&'a str, &'a str>,
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
        headers.insert(Header::ContentType.as_str(), content_type.as_str());

        Response {
            status,
            headers,
            body,
            should_close_connection,
        }
    }

    pub fn write(&self, tcp_stream: &mut TcpStream) {
        let mut metadata = String::with_capacity(128);
        metadata.push_str(self.status.as_str());

        for (header_name, header_value) in &self.headers {
            metadata.push_str(&format!("{header_name}: {header_value}\r\n"))
        }

        if !self.headers.contains_key(&Header::ContentLength.as_str()) {
            metadata.push_str(&format!(
                "{}: {}\r\n",
                Header::ContentLength.as_str(),
                self.body.len()
            ));
        }
        
        if self.should_close_connection
            && !self.headers.contains_key(&Header::Connection.as_str())
        {
            metadata.push_str(&format!(
                "{}: {}\r\n",
                Header::Connection.as_str(),
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
