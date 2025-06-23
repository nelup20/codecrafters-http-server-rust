
pub enum Status {
    Ok,
    Created,
    NotFound,
    BadRequest
}

impl Status {
    pub fn as_string(&self) -> &str {
        match self {
            Status::Ok => "HTTP/1.1 200 OK\r\n",
            Status::Created => "HTTP/1.1 201 Created\r\n",
            Status::NotFound => "HTTP/1.1 404 Not Found\r\n",
            Status::BadRequest => "HTTP/1.1 400 Bad Request\r\n"
        }
    }
}