pub(crate) mod body;
#[cfg(test)]
mod cfg;
pub(crate) mod compress;
pub(crate) mod constant;
pub(crate) mod content_type;
pub(crate) mod global_trait;
pub(crate) mod http_url;
pub(crate) mod http_version;
pub(crate) mod methods;
pub(crate) mod protocol;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod status_code;
pub(crate) mod utils;

pub use request::{
    error::r#type::Error, r#trait::Request, r#type::BoxHttpRequest,
    request_builder::r#type::RequestBuilder,
};
pub use response::{r#trait::Response, r#type::BoxHttpResponse};
