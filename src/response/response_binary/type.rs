use std::collections::HashMap;

/// A struct representing an HTTP response.
///
/// This struct contains all the components of an HTTP response: the HTTP version, status code,
/// status text, headers, and body. It is used to model and manipulate HTTP responses within the
/// application.
///
/// # Fields
/// - `http_version`: A string representing the HTTP version (e.g., "HTTP/1.1").
/// - `status_code`: The HTTP status code (e.g., 200 for OK, 404 for Not Found).
/// - `status_text`: A string containing the status text associated with the status code (e.g., "OK", "Not Found").
/// - `headers`: A `HashMap<String, String>` containing the headers of the response, where each key is the header name
///   (e.g., "Content-Type"), and the value is the corresponding header value.
/// - `body`: A `Vec<u8>` representing the body of the HTTP response, which contains the content being returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponseBinary {
    /// The HTTP version of the response (e.g., "HTTP/1.1").
    pub http_version: String,

    /// The HTTP status code (e.g., 200, 404).
    pub status_code: u16,

    /// The status text associated with the status code (e.g., "OK", "Not Found").
    pub status_text: String,

    /// A `HashMap` of headers, where the key is the header name and the value is the header value.
    pub headers: HashMap<String, String>,

    /// The body of the response, which contains the content being returned.
    pub body: Vec<u8>,
}
