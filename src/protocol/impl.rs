use super::r#type::Protocol;
use std::{
    fmt::{self, Display},
    str::FromStr,
};
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
    #[allow(dead_code)]
    pub fn new() -> Self {
        Protocol::default()
    }

    /// Checks if the current protocol is `HTTP`.
    ///
    /// Returns `true` if the protocol is `HTTP`, otherwise returns `false`.
    #[allow(dead_code)]
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
            Protocol::Unknown(_) => 80,
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
            Protocol::Unknown(protocol) => protocol,
        };
        write!(f, "{}", res)
    }
}

impl FromStr for Protocol {
    type Err = &'static str;

    /// Parses a string into a `Protocol` variant.
    ///
    /// This method attempts to convert a string into one of the `Protocol` variants:
    /// - "http" will be converted to `Protocol::HTTP`.
    /// - "https" will be converted to `Protocol::HTTPS`.
    ///
    /// If the string doesn't match either of these values, it returns an error.
    ///
    /// # Parameters
    /// - `s`: A string slice representing the protocol to be parsed.
    ///
    /// # Returns
    /// A `Result` that is either:
    /// - `Ok(Protocol::HTTP)` if the string is "http".
    /// - `Ok(Protocol::HTTPS)` if the string is "https".
    /// - `Err("Invalid protocol")` if the string doesn't match either value.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "http" => Ok(Protocol::HTTP),
            "https" => Ok(Protocol::HTTPS),
            _ => Ok(Protocol::Unknown(s.to_string())),
        }
    }
}
