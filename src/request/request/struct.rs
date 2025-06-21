use crate::*;

/// Represents an HTTP request, encapsulating various components such as the method, URL, protocol,
/// headers, body, and additional metadata.
#[derive(Debug, Clone)]
pub(crate) struct HttpRequest {
    pub(crate) methods: Arc<Method>,
    pub(crate) url: Arc<String>,
    pub(crate) header: Arc<RequestHeaders>,
    pub(crate) body: Arc<Body>,
    pub(crate) config: ArcRwLock<Config>,
    pub(crate) tmp: ArcRwLock<Tmp>,
    pub(crate) response: ArcRwLock<HttpResponseBinary>,
}
