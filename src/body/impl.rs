use super::r#type::Body;
use serde::{Serialize, Serializer};
use std::fmt::{self};

/// Represents the body of a request or response, which can be either plain text, JSON, or binary data.
impl Default for Body {
    /// Provides a default implementation for the `Body` enum.
    ///
    /// Returns a `Body::Text` variant with an empty string as the default body.
    fn default() -> Self {
        Self::Text("")
    }
}

/// Implements the `fmt::Display` trait for the `Body` enum.
///
/// This trait is responsible for providing a human-readable representation of the body:
/// - For the `Text` variant, the contained text is displayed.
/// - For the `Json` variant, the JSON is serialized to a string. If serialization fails,
///   an empty JSON object (`{}`) is displayed.
/// - For the `Binary` variant, the binary data is displayed in a debug format.
impl fmt::Display for Body {
    /// Formats the `Body` instance for user-friendly display.
    ///
    /// # Parameters
    /// - `f`: A mutable reference to the formatter.
    ///
    /// # Returns
    /// A `Result` indicating whether the formatting was successful. The result is
    /// `fmt::Result` which is either `Ok` or an error.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::Text(text) => write!(f, "{}", text.to_string()),
            Body::Json(json) => write!(
                f,
                "{:?}",
                serde_json::to_string(json).unwrap_or_else(|_| String::from("{}"))
            ),
            Body::Binary(binary) => write!(f, "{:?}", binary),
        }
    }
}

/// Implements the `Serialize` trait for the `Body` enum.
///
/// This trait enables the serialization of the `Body` enum into different formats using
/// the `serde` framework:
/// - The `Text` variant is serialized as a plain string.
/// - The `Json` variant is serialized using the provided `serde` serializer.
/// - The `Binary` variant is serialized as binary data.
impl Serialize for Body {
    /// Serializes the `Body` instance into the given serializer.
    ///
    /// # Type Parameters
    /// - `S`: The serializer type implementing the `Serializer` trait.
    ///
    /// # Parameters
    /// - `serializer`: The serializer used for serialization, which will handle
    ///   the transformation of the `Body` into the target format.
    ///
    /// # Returns
    /// A `Result` containing either the successfully serialized output or an error.
    /// If successful, it returns `S::Ok`; if an error occurs, it returns `S::Error`.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Body::Text(text) => text.serialize(serializer),
            Body::Json(json) => json.serialize(serializer),
            Body::Binary(binary) => binary.serialize(serializer),
        }
    }
}
