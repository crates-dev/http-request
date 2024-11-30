use std::collections::HashMap;

/// Represents the body of an HTTP request as a JSON object, stored as a `HashMap`
/// with static string keys and values. This type is typically used for structured data
/// in the body of the request.
pub type BodyJson = HashMap<&'static str, &'static str>;

/// Represents the body of an HTTP request as plain text. This type is used for requests
/// where the body contains raw string content, such as simple text data or form submissions.
pub type BodyText = &'static str;

/// Represents the body of an HTTP request, which can be either plain text or JSON.
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    /// The text variant of the body, containing plain string content.
    Text(BodyText),

    /// The JSON variant of the body, containing structured JSON data.
    Json(BodyJson),
}
