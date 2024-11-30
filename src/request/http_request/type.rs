use std::sync::Arc;

use crate::{
    global_type::r#type::{Body, Header},
    methods::r#type::Methods,
    protocol::r#type::Protocol,
};

/// Represents an HTTP request with methods, URL, protocol, headers, body, and timeout settings.
///
/// The `HttpRequest` struct is designed to encapsulate all the necessary information for
/// making an HTTP request. This includes the HTTP method, the target URL, the protocol,
/// the request headers, the request body, and the timeout setting. All fields are wrapped
/// in `Arc` to allow for safe sharing across threads, ensuring that the data remains
/// immutable and efficiently shared.
///
/// # Fields
///
/// - `methods`: The HTTP method to be used for the request, such as GET, POST, etc.
/// - `url`: The target URL for the request.
/// - `protocol`: The protocol to be used (e.g., HTTP/1.1, HTTP/2, etc.).
/// - `header`: The headers associated with the HTTP request.
/// - `body`: The body content of the HTTP request.
/// - `timeout`: The timeout duration (in milliseconds) for the HTTP request.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub methods: Arc<Methods>,
    pub url: Arc<String>,
    pub protocol: Arc<Protocol>,
    pub header: Arc<Header>,
    pub body: Arc<Body>,
    pub timeout: Arc<u64>,
}
