use super::{response_binary::r#type::HttpResponseBinary, response_text::r#type::HttpResponseText};
use crate::Response;

/// A type alias for a boxed dynamic trait object implementing the `Response` trait.
///
/// This alias defines a `Response` as a `Box` containing any type that implements the
/// `Response` trait, with associated types `OutputText` set to `HttpResponseText`
/// and `OutputBinary` set to `HttpResponseBinary`. It allows for flexible handling of
/// HTTP responses that can be either in text or binary format.
pub type BoxHttpResponse =
    Box<dyn Response<OutputText = HttpResponseText, OutputBinary = HttpResponseBinary>>;
