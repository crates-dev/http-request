pub(crate) mod global_type;
pub(crate) mod methods;
pub(crate) mod protocol;
pub(crate) mod request;
pub(crate) mod request_url;
pub(crate) mod response;

pub use methods::r#type::Methods;
pub use protocol::r#type::Protocol;
pub use request::r#type::{HttpRequest, HttpRequestBuilder};
pub use std_macro_extensions::*;
