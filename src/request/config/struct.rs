use crate::*;

/// Configuration for HTTP requests.
///
/// This struct holds various settings and state information required for making HTTP requests.
/// It allows the user to configure timeouts, URL, redirection handling, and HTTP version,
/// as well as to manage the request's buffer size and redirection behavior.
///
/// # Fields
///
/// - `timeout` - The timeout duration for the request in milliseconds.  
///   Requests exceeding this duration will be terminated.
///
/// - `url_obj` - The parsed URL object containing the details of the request destination.  
///   This field represents the URL that the request will be sent to, parsed into a `HttpUrlComponents` object.
///
/// - `redirect` - A flag indicating whether redirection is enabled for the request.  
///   If `true`, the client will follow HTTP redirections (e.g., 3xx status codes).  
///   If `false`, the client will not follow redirections and will return the response as is.
///
/// - `max_redirect_times` - The maximum number of allowed redirections.  
///   A value of `0` disables redirection entirely.  
///   If redirections exceed this number, the client will return an error.
///
/// - `redirect_times` - The current count of redirections followed during this request.  
///   This field is dynamically updated as the request is processed, and can be used to track the
///   number of redirections that have occurred.
///
/// - `http_version` - The version of HTTP to use for the request, stored as an `HttpVersion` type.  
///   This field defines which HTTP version (e.g., HTTP/1.1 or HTTP/2) to use for the request.
///
/// - `buffer` - The size of the buffer to use for reading the HTTP response.  
///   This defines the number of bytes that can be read into memory at a time when processing the response.
///
/// # Notes
///
/// - Ensure that the `timeout` value is appropriate for your application's requirements to avoid
///   prematurely terminating requests.
///
/// - Properly configure the `max_redirect_times` to avoid infinite loops in cases of cyclic redirections.
///   If your application requires following redirects, ensure that the value is set appropriately (e.g., 5).
///
/// - The `http_version` field should be configured according to the supported version for your application.
///   If not set properly, it could result in requests using an incompatible version of HTTP.
///
/// - Adjust the `buffer` size depending on your application’s memory usage and the size of the responses
///   you expect to receive. A larger buffer can help improve performance when handling large responses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Config {
    /// The timeout duration for the request in milliseconds.
    pub(crate) timeout: u64,

    /// The parsed URL object containing the details of the request destination.
    pub(crate) url_obj: HttpUrlComponents,

    /// A flag indicating whether redirection is enabled for the request.
    pub(crate) redirect: bool,

    /// The maximum number of allowed redirections.
    pub(crate) max_redirect_times: usize,

    /// The current count of redirections followed during this request.
    pub(crate) redirect_times: usize,

    /// The type of this field is `HttpVersion`.
    pub(crate) http_version: HttpVersion,

    /// The buffer size for reading the HTTP response and decode.
    pub(crate) buffer: usize,

    /// Auto decode response data
    pub(crate) decode: bool,
}
