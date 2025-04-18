use crate::*;

/// Represents an HTTP request, encapsulating various components such as the method, URL, protocol,
/// headers, body, and additional metadata.
#[derive(Debug, Clone)]
pub(crate) struct HttpRequest {
    /// The HTTP method of the request (e.g., GET, POST, etc.).
    pub(crate) methods: Arc<Methods>,

    /// The target URL of the request.
    pub(crate) url: Arc<String>,

    /// The headers included in the request.
    pub(crate) header: Arc<RequestHeaders>,

    /// The type of the body, specifying whether it is text or JSON.
    pub(crate) body: Arc<Body>,

    /// Represents the configuration settings for the HTTP request.
    pub(crate) config: ArcRwLock<Config>,

    /// Stores temporary data during the HTTP request process.
    pub(crate) tmp: ArcRwLock<Tmp>,

    /// Http response
    pub(crate) response: ArcRwLock<HttpResponseBinary>,
}
