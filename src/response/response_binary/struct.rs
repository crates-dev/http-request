use crate::*;

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
#[derive(Debug, Clone)]
pub struct HttpResponseBinary {
    /// HTTP protocol version.
    pub(crate) http_version: ArcRwLock<HttpVersion>,
    /// HTTP response status code.
    pub(crate) status_code: ResponseStatusCode,
    /// HTTP response status text.
    pub(crate) status_text: ArcRwLock<String>,
    /// HTTP response headers.
    pub(crate) headers: ArcRwLock<ResponseHeaders>,
    /// HTTP response body content.
    pub(crate) body: ArcRwLock<RequestBody>,
}
