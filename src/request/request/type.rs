use crate::*;

pub type RequestResult = Result<BoxResponseTrait, RequestError>;
pub type BoxAsyncRequestTrait = Box<dyn AsyncRequestTrait<RequestResult = RequestResult>>;
pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
pub(crate) type BoxAsyncReadWrite = Box<dyn AsyncReadWrite>;
pub(crate) type BoxReadWrite = Box<dyn ReadWrite>;
