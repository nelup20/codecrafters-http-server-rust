use crate::content_type::ContentType;

pub enum Header {
    ContentType(ContentType),
    ContentLength(usize),
    UserAgent(String),
}

impl Header {
    pub fn as_string(&self) -> String {
        match self {
            Header::ContentType(val) => format!("Content-Type: {}\r\n", val.as_string()),
            Header::ContentLength(val) => format!("Content-Length: {}\r\n", val),
            Header::UserAgent(val) => format!("User-Agent: {}\r\n", val),
        }
    }
}
