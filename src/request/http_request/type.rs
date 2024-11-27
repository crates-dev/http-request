use std::sync::Arc;

use crate::{
    global_type::r#type::{Body, Header},
    Methods, Protocol,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub methods: Arc<Methods>,
    pub url: Arc<String>,
    pub protocol: Arc<Protocol>,
    pub header: Arc<Header>,
    pub body: Arc<Body>,
}
