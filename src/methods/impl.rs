use std::fmt::{self, Display};

use super::r#type::Methods;

/// Implements the default value for `Methods`.
///
/// The default value is `Methods::GET`, which serves as the default HTTP request method.
impl Default for Methods {
    fn default() -> Self {
        Methods::GET
    }
}

/// Provides utility methods for the `Methods` type.
impl Methods {
    /// Creates a new instance of `Methods` with the default value of `Methods::GET`.
    ///
    /// This is a shorthand for using the `default` method.
    pub fn new() -> Self {
        Methods::default()
    }

    /// Checks if the current method is `GET`.
    ///
    /// Returns `true` if the method is `GET`, otherwise returns `false`.
    pub fn is_get(&self) -> bool {
        self.to_owned() == Methods::GET.to_owned()
    }

    /// Checks if the current method is `POST`.
    ///
    /// Returns `true` if the method is `POST`, otherwise returns `false`.
    pub fn is_post(&self) -> bool {
        self.to_owned() == Methods::POST.to_owned()
    }
}

/// Implements the `Display` trait for `Methods`.
///
/// Formats the `Methods` enum as a string, returning `"GET"` for `Methods::GET`
/// and `"POST"` for `Methods::POST`.
impl Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Methods::GET => "GET",
            Methods::POST => "POST",
        };
        write!(f, "{}", res)
    }
}
