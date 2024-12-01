/// Represents the HTTP version used in the request or response.
///
/// This enum is used to specify the HTTP version for HTTP requests and responses.
/// It supports the two most common HTTP versions: HTTP/1.1 and HTTP/2. The `HttpVersion`
/// enum allows for easy comparison, cloning, and debugging of the HTTP version.
///
/// The variants include:
/// - `HTTP1_1`: Represents HTTP version 1.1.
/// - `HTTP2`: Represents HTTP version 2.0.
///
/// # Derives
/// The enum derives the following traits:
/// - `Debug`: Allows for formatting the enum for debugging purposes.
/// - `PartialEq`: Allows for comparing instances of `HttpVersion` for equality.
/// - `Clone`: Allows for creating a copy of `HttpVersion` instances.
#[derive(Debug, PartialEq, Clone)]
pub enum HttpVersion {
    /// HTTP version 1.1
    HTTP1_1,

    /// HTTP version 2.0
    HTTP2,

    /// Unknown version
    Unknown(String),
}
