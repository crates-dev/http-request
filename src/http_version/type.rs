/// Represents the HTTP version used in the request or response.
///
/// This enum is used to specify the HTTP version for HTTP requests and responses.
/// It supports the two most common HTTP versions: HTTP/1.1 and HTTP/2. The `HttpVersion`
/// enum allows for easy comparison, cloning, and debugging of the HTTP version.
///
/// The variants include:
/// - `HTTP1_1`: Represents HTTP version 1.1.
/// - `HTTP2`: Represents HTTP version 2.0.
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum HttpVersion {
    /// HTTP version 1.1
    HTTP1_1,

    /// HTTP version 2.0
    HTTP2,

    /// Unknown version
    Unknown(String),
}
