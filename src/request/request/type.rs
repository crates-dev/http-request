use crate::{
    body::r#type::Body,
    request::{config::r#type::Config, tmp::r#type::Tmp},
    response::response_binary::r#type::HttpResponseBinary,
};
use http_type::*;
use std::sync::Arc;

/// Represents an HTTP request, encapsulating various components such as the method, URL, protocol,
/// headers, body, and additional metadata.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// The HTTP method of the request (e.g., GET, POST, etc.).
    pub methods: Arc<Methods>,

    /// The target URL of the request.
    pub url: Arc<String>,

    /// The headers included in the request.
    pub header: Arc<HttpHeaderSliceMap>,

    /// The type of the body, specifying whether it is text or JSON.
    pub body: Arc<Body>,

    /// Represents the configuration settings for the HTTP request.
    pub config: Config,

    /// Stores temporary data during the HTTP request process.
    pub tmp: Tmp,

    /// Http response
    pub response: HttpResponseBinary,
}
