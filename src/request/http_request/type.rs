use crate::{
    body::r#type::Body, header::r#type::Header, methods::r#type::Methods,
    protocol::r#type::Protocol, request::config::r#type::Config,
};
use std::sync::Arc;
/// Represents an HTTP request, encapsulating various components such as the method, URL, protocol,
/// headers, body, and additional metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    /// The HTTP method of the request (e.g., GET, POST, etc.).
    pub methods: Arc<Methods>,

    /// The target URL of the request.
    pub url: Arc<String>,

    /// The protocol version used in the request (e.g., HTTP/1.1, HTTP/2.0).
    pub protocol: Arc<Protocol>,

    /// The headers included in the request.
    pub header: Arc<Header>,

    /// The type of the body, specifying whether it is text or JSON.
    pub body: Arc<Body>,

    pub config: Config,
}
