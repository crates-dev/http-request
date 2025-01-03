use super::r#trait::RequestTrait;
use crate::response::r#type::BoxResponseTrait;
use http_type::*;

/// Type alias for `Result<BoxResponseTrait, RequestError>`
pub type RequestResult = Result<BoxResponseTrait, RequestError>;

/// Type alias for a boxed trait object implementing `RequestTrait`
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
