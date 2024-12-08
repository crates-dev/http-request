use super::r#trait::Request;
use crate::request::error::r#type::Error;
use crate::response::r#type::BoxHttpResponse;
/// Type alias for `Result<BoxHttpResponse, Error>`
pub type RequestResult = Result<BoxHttpResponse, Error>;

/// Type alias for a boxed trait object implementing `Request`
pub type BoxHttpRequest = Box<dyn Request<RequestResult = RequestResult>>;
