use super::{
    http_response_binary::r#type::HttpResponseBinary, http_response_text::r#type::HttpResponseText,
};
use crate::HttpResponse;

/// A type alias for a boxed dynamic trait object implementing the `HttpResponse` trait.
///
/// This alias defines a `Response` as a `Box` containing any type that implements the
/// `HttpResponse` trait, with associated types `OutputText` set to `HttpResponseText`
/// and `OutputBinary` set to `HttpResponseBinary`. It allows for flexible handling of
/// HTTP responses that can be either in text or binary format.
pub type BoxHttpResponse =
    Box<dyn HttpResponse<OutputText = HttpResponseText, OutputBinary = HttpResponseBinary>>;
