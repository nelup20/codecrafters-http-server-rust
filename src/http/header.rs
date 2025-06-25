
pub enum Header {
    AcceptEncoding,
    Connection,
    ContentType,
    ContentEncoding,
    ContentLength,
    UserAgent,
}

impl Header {
    pub fn as_str(&self) -> &str {
        match self {
            Header::AcceptEncoding => "Accept-Encoding",
            Header::Connection => "Connection",
            Header::ContentType => "Content-Type",
            Header::ContentEncoding => "Content-Encoding",
            Header::ContentLength => "Content-Length",
            Header::UserAgent => "User-Agent",
        }
    }
}
