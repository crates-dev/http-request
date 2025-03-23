use crate::*;

/// Represents the body of an HTTP request as a JSON object, stored as a `HashMapXxHash3_64`
/// with static string keys and values. This type is typically used for structured data
/// in the body of the request.
pub type BodyJson = HashMapXxHash3_64<&'static str, &'static str>;

/// Represents the body of an HTTP request as plain text. This type is used for requests
/// where the body contains raw string content, such as simple text data or form submissions.
pub type BodyText = &'static str;

pub type BodyBinary = &'static [u8];

/// Represents the body of an HTTP request, which can be either plain text, JSON, or binary data.
/// This enum allows different types of content to be used in the body of the request, supporting
/// both structured and unstructured data formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    /// The text variant of the body, containing plain string content.
    ///
    /// This variant holds a reference to a static string (`BodyText`), typically used
    /// when the request body contains text data such as a plain message or form input.
    Text(BodyText),

    /// The JSON variant of the body, containing structured JSON data.
    ///
    /// This variant holds a `BodyJson`, which is a `HashMapXxHash3_64` with static string keys
    /// and values. It is useful for sending data in JSON format, typically for APIs
    /// that require structured data in the request body.
    Json(BodyJson),

    /// The binary variant of the body, containing raw binary data.
    ///
    /// This variant holds a reference to a static slice of bytes (`BodyBinary`), and
    /// is useful when sending raw binary data such as images, files, or non-text content.
    Binary(BodyBinary),
}
