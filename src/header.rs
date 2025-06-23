use crate::compression_scheme::CompressionScheme;
use crate::content_type::ContentType;

pub enum Header {
    ContentType(ContentType),
    ContentEncoding(CompressionScheme),
    ContentLength(usize),
    UserAgent(String),
}

impl Header {
    pub fn as_string(&self) -> String {
        match self {
            Header::ContentType(val) => format!("Content-Type: {}\r\n", val.as_string()),
            Header::ContentEncoding(val) => format!("Content-Encoding: {}\r\n", val.as_string()),
            Header::ContentLength(val) => format!("Content-Length: {}\r\n", val),
            Header::UserAgent(val) => format!("User-Agent: {}\r\n", val),
        }
    }
}
