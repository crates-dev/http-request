use crate::*;

/// Type alias for `Result<BoxResponseTrait, RequestError>`
pub type RequestResult = Result<BoxResponseTrait, RequestError>;

/// Type alias for a boxed trait object implementing `RequestTrait`
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
