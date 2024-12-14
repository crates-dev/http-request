#[derive(Debug, PartialEq, Eq)]
pub enum Compress {
    Gzip,
    Deflate,
    Br,
    Unknown,
}
