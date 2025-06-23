
pub enum ContentType {
    TextPlain,
    OctetStream
}

impl ContentType {
    pub fn as_string(&self) -> &str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::OctetStream => "application/octet-stream"
        }
    }
}