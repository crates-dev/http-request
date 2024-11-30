use std::fmt;

use serde::{Serialize, Serializer};

use super::r#type::Body;

/// Represents the body of a request or response, which can be either plain text or JSON.
impl Default for Body {
    /// Provides a default implementation for the `Body` enum.
    ///
    /// Returns a `Body::Text` variant with an empty string.
    fn default() -> Self {
        Self::Text("")
    }
}

/// Implements the `fmt::Display` trait for the `Body` enum.
///
/// Formats the body for display purposes:
/// - For the `Text` variant, it outputs the contained text.
/// - For the `Json` variant, it attempts to serialize the JSON value to a string.
///   If serialization fails, it defaults to an empty JSON object (`{}`).
impl fmt::Display for Body {
    /// Formats the `Body` instance for user-friendly display.
    ///
    /// # Parameters
    /// - `f`: A mutable reference to the formatter.
    ///
    /// # Returns
    /// A `Result` indicating whether the formatting was successful.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Text(text) => write!(f, "{}", text.to_string()),
            Body::Json(json) => write!(
                f,
                "{:?}",
                serde_json::to_string(json).unwrap_or_else(|_| String::from("{}"))
            ),
        }
    }
}

/// Implements the `Serialize` trait for the `Body` enum.
///
/// Supports serialization of both text and JSON variants:
/// - The `Text` variant is serialized as a plain string.
/// - The `Json` variant is serialized using the provided `serde` serializer.
impl Serialize for Body {
    /// Serializes the `Body` instance into the given serializer.
    ///
    /// # Type Parameters
    /// - `S`: The serializer type implementing the `Serializer` trait.
    ///
    /// # Parameters
    /// - `serializer`: The serializer used for serialization.
    ///
    /// # Returns
    /// A `Result` containing either the successfully serialized output or an error.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Body::Text(text) => text.serialize(serializer),
            Body::Json(json) => json.serialize(serializer),
        }
    }
}
