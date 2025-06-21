use crate::*;

pub type RequestResult = Result<BoxResponseTrait, RequestError>;

pub type BoxAsyncRequestTrait = Box<dyn AsyncRequestTrait<RequestResult = RequestResult>>;

pub type BoxRequestTrait = Box<dyn RequestTrait<RequestResult = RequestResult>>;
