
pub enum Status {
    Ok,
    NotFound
}

impl Status {
    pub fn as_string(&self) -> &str {
        match self {
            Status::Ok => "HTTP/1.1 200 OK\r\n",
            Status::NotFound => "HTTP/1.1 404 Not Found\r\n"
        }
    }
}