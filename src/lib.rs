pub(crate) mod body;
#[cfg(test)]
mod cfg;
pub(crate) mod constant;
pub(crate) mod global_trait;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod utils;

pub use http_type::RequestError;
pub use request::{
    r#trait::RequestTrait, r#type::BoxRequestTrait, request::r#type::HttpRequest,
    request_builder::r#type::RequestBuilder,
};
pub use response::{
    r#trait::ResponseTrait, r#type::BoxResponseTrait, response_binary::r#type::HttpResponseBinary,
    response_text::r#type::HttpResponseText,
};
