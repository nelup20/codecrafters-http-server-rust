
pub enum Header {
    AcceptEncoding,
    Connection,
    ContentType,
    ContentEncoding,
    ContentLength,
    UserAgent,
}

impl Header {
    pub fn as_string(&self) -> String {
        match self {
            Header::AcceptEncoding => String::from("Accept-Encoding"),
            Header::Connection => String::from("Connection"),
            Header::ContentType => String::from("Content-Type"),
            Header::ContentEncoding => String::from("Content-Encoding"),
            Header::ContentLength => String::from("Content-Length"),
            Header::UserAgent => String::from("User-Agent"),
        }
    }
}
