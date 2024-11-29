use std::sync::Arc;

use crate::{
    global_type::r#type::{Body, Header},
    Methods, Protocol,
};

/// Represents an HTTP request.
///
/// This struct defines the structure of an HTTP request, including its method, URL,
/// protocol, headers, and body.
///
/// # Fields
/// - `methods`: The HTTP method of the request (e.g., GET, POST), stored as a shared reference.
/// - `url`: The target URL of the request, stored as a shared reference.
/// - `protocol`: The protocol used for the request (e.g., HTTP/1.1), stored as a shared reference.
/// - `header`: The HTTP headers associated with the request, stored as a shared reference.
/// - `body`: The body of the HTTP request, stored as a shared reference.
///
/// # Traits
/// - `Debug`: Enables formatted printing for debugging purposes.
/// - `Clone`: Allows for cloning of the struct and its fields.
/// - `PartialEq`: Provides equality comparison between two instances of the struct.
///
/// This struct is designed for use in building and processing HTTP requests in
/// a thread-safe and efficient manner.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub methods: Arc<Methods>,
    pub url: Arc<String>,
    pub protocol: Arc<Protocol>,
    pub header: Arc<Header>,
    pub body: Arc<Body>,
}
