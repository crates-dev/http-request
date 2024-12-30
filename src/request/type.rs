use super::r#trait::RequestTrait;
use crate::request::error::r#type::Error;
use crate::response::r#type::BoxResponseTrait;

/// Type alias for `Result<BoxResponseTrait, Error>`
pub type RequestResult = Result<BoxResponseTrait, Error>;

/// Type alias for a boxed trait object implementing `RequestTrait`
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
