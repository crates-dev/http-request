use http_type::*;

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
pub struct HttpResponseText {
    /// The HTTP version of the response (e.g., "HTTP/1.1").
    pub(crate) http_version: ArcRwLock<HttpVersion>,

    /// The HTTP status code (e.g., 200, 404).
    pub(crate) status_code: StatusCodeUsize,

    /// The status text associated with the status code (e.g., "OK", "Not Found").
    pub(crate) status_text: ArcRwLock<String>,

    /// A `HashMap` of headers, where the key is the header name and the value is the header value.
    pub(crate) headers: ArcRwLock<HttpHeaderMap>,

    /// The body of the response, which contains the content being returned.
    pub(crate) body: ArcRwLock<HttpBodyString>,
}
