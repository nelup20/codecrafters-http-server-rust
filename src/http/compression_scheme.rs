
pub enum CompressionScheme {
    Gzip
}

impl CompressionScheme {
    pub fn as_string(&self) -> String {
        match self {
            CompressionScheme::Gzip => String::from("gzip"),
        }
    }
}