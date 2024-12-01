use crate::request::{http_version::r#type::HttpVersion, request_url::r#type::RequestUrl};

/// Configuration for HTTP requests.
///
/// This struct holds various settings and state information required for making HTTP requests.
///
/// # Fields
///
/// - `timeout` - The timeout duration for the request in milliseconds.  
///   Requests exceeding this duration will be terminated.  
///
/// - `url_obj` - The parsed URL object containing the details of the request destination.  
///
/// - `redirect` - A flag indicating whether redirection is enabled for the request.  
///   If `true`, the client will follow HTTP redirections.  
///
/// - `max_redirect_times` - The maximum number of allowed redirections.  
///   A value of `0` disables redirection entirely.  
///
/// - `redirect_times` - The current count of redirections followed during this request.  
///   This value is updated dynamically as the request is processed.  
///
/// # Notes
///
/// - Ensure that the `timeout` value is appropriate for your application's requirements.  
/// - Properly configure the `max_redirect_times` to avoid infinite loops in cases of cyclic redirections.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    /// The timeout duration for the request in milliseconds.
    pub timeout: u64,

    /// The parsed URL object containing the details of the request destination.
    pub url_obj: RequestUrl,

    /// A flag indicating whether redirection is enabled for the request.
    pub redirect: bool,

    /// The maximum number of allowed redirections.
    pub max_redirect_times: usize,

    /// The current count of redirections followed during this request.
    pub redirect_times: usize,

    /// The type of this field is `HttpVersion`.
    pub http_version: HttpVersion,
}
