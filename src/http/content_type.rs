
pub enum ContentType {
    TextPlain,
    OctetStream
}

impl ContentType {
    pub fn as_string(&self) -> String {
        match self {
            ContentType::TextPlain => String::from("text/plain"),
            ContentType::OctetStream => String::from("application/octet-stream")
        }
    }
}