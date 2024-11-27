use crate::{
    global_type::r#type::{Body, Header},
    methods::r#type::Methods,
    protocol::r#type::Protocol,
};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub methods: Arc<Methods>,
    pub url: Arc<String>,
    pub protocol: Arc<Protocol>,
    pub header: Arc<Header>,
    pub body: Arc<Body>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequestBuilder {
    pub tmp: HttpRequest,
    pub builder: HttpRequest,
}
