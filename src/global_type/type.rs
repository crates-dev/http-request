use std::collections::HashMap;

/// A type alias for a `HashMap<String, String>`, representing the body of an HTTP request or response.
/// This structure is typically used to hold key-value pairs where the key is a string representing
/// the content type (e.g., form data key) and the value is the corresponding string content.
pub type Body = HashMap<String, String>;

/// A type alias for a `HashMap<String, String>`, representing the headers of an HTTP request or response.
/// This structure stores key-value pairs, where each key is the name of an HTTP header (e.g., `Content-Type`, `Host`),
/// and the value is the corresponding header value (e.g., `application/json`, `example.com`).
pub type Header = HashMap<String, String>;
