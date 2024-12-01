use super::r#type::HttpVersion;
use crate::constant::http::{HTTP_VERSION_1_1, HTTP_VERSION_2};
use std::fmt;

impl Default for HttpVersion {
    /// Provides the default value for `HttpVersion`.
    ///
    /// This method initializes a `HttpVersion` instance with the default value, which is
    /// `HttpVersion::HTTP1_1`. This is typically used when a `HttpVersion` instance is created
    /// and no specific version is provided, falling back to HTTP/1.1 as the default.
    ///
    /// # Returns
    /// Returns the default `HttpVersion::HTTP1_1`.
    fn default() -> Self {
        Self::HTTP1_1
    }
}

impl fmt::Display for HttpVersion {
    /// Formats the `HttpVersion` instance as a string.
    ///
    /// This method converts the `HttpVersion` instance into its corresponding string representation,
    /// which can be useful for displaying or logging the HTTP version. The version is returned as a
    /// human-readable string based on the variant of the `HttpVersion`.
    ///
    /// # Parameters
    /// - `f`: The formatter to write the output to.
    ///
    /// # Returns
    /// Returns a `fmt::Result`, indicating the success or failure of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version_str = match self {
            HttpVersion::HTTP1_1 => HTTP_VERSION_1_1,
            HttpVersion::HTTP2 => HTTP_VERSION_2,
        };
        write!(f, "{}", version_str)
    }
}
