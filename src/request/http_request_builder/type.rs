use crate::request::http_request::r#type::HttpRequest;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequestBuilder {
    pub(crate) tmp: HttpRequest,
    pub(crate) builder: HttpRequest,
}
