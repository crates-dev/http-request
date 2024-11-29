use super::r#type::Protocol;
use std::fmt::{self, Display};
/// Implements the default value for `Protocol`.
///
/// The default value is `Protocol::HTTP`.
impl Default for Protocol {
    fn default() -> Self {
        Protocol::HTTP
    }
}

/// Provides utility methods for the `Protocol` type.
impl Protocol {
    /// Creates a new instance of `Protocol` with the default value of `Protocol::HTTP`.
    ///
    /// This is a shorthand for using the `default` method.
    pub fn new() -> Self {
        Protocol::default()
    }

    /// Checks if the current protocol is `HTTP`.
    ///
    /// Returns `true` if the protocol is `HTTP`, otherwise returns `false`.
    pub fn is_http(&self) -> bool {
        self.to_owned() == Protocol::HTTP.to_owned()
    }

    /// Checks if the current protocol is `HTTPS`.
    ///
    /// Returns `true` if the protocol is `HTTPS`, otherwise returns `false`.
    pub fn is_https(&self) -> bool {
        self.to_owned() == Protocol::HTTPS.to_owned()
    }

    /// Returns the default port associated with the protocol.
    ///
    /// - Returns `80` for `Protocol::HTTP`.
    /// - Returns `443` for `Protocol::HTTPS`.
    pub fn get_port(&self) -> u16 {
        match self {
            Protocol::HTTP => 80,
            Protocol::HTTPS => 443,
        }
    }
}

/// Implements the `Display` trait for `Protocol`.
///
/// Formats the `Protocol` enum as a lowercase string, returning:
/// - `"http"` for `Protocol::HTTP`.
/// - `"https"` for `Protocol::HTTPS`.
impl Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Protocol::HTTP => "http",
            Protocol::HTTPS => "https",
        };
        write!(f, "{}", res)
    }
}
