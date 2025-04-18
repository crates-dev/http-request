use crate::*;

/// Represents the body of an HTTP request as a JSON object, stored as a `HashMapXxHash3_64`
/// with const string keys and values. This type is typically used for structured data
/// in the body of the request.
pub type BodyJson = HashMapXxHash3_64<&'static str, &'static str>;

/// Represents the body of an HTTP request as plain text. This type is used for requests
/// where the body contains raw string content, such as simple text data or form submissions.
pub type BodyText = &'static str;

/// Represents the body of an HTTP request as binary data. This type is used for requests
/// where the body contains raw bytes, such as file uploads, images, or other non-text content.
pub type BodyBinary = &'static [u8];
