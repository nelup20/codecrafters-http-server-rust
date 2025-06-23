use crate::header::Header::{ContentLength, ContentType, UserAgent};

pub enum Header {
    ContentType,
    ContentLength(usize),
    UserAgent(String),
}

impl Header {
    pub fn as_string(&self) -> String {
        match self {
            ContentType => String::from("Content-Type: text/plain\r\n"),
            ContentLength(val) => format!("Content-Length: {}\r\n", val),
            UserAgent(val) => format!("User-Agent: {}\r\n", val),
        }
    }
}
