
pub enum CompressionScheme {
    Gzip
}

impl CompressionScheme {
    pub fn as_str(&self) -> &str {
        match self {
            CompressionScheme::Gzip => "gzip",
        }
    }
}