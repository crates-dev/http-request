/// Byte pattern for matching 'content-length' header in HTTP requests.
///
/// Used for case-sensitive matching of the content-length header.
pub(crate) const CONTENT_LENGTH_PATTERN: &[u8] = b"content-length:";
