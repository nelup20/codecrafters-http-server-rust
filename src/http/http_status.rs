
pub enum HttpStatus {
    Ok,
    Created,
    NotFound,
    BadRequest
}

impl HttpStatus {
    pub fn as_str(&self) -> &str {
        match self {
            HttpStatus::Ok => "HTTP/1.1 200 OK\r\n",
            HttpStatus::Created => "HTTP/1.1 201 Created\r\n",
            HttpStatus::NotFound => "HTTP/1.1 404 Not Found\r\n",
            HttpStatus::BadRequest => "HTTP/1.1 400 Bad Request\r\n"
        }
    }
}