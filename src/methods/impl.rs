// TODO:
use super::r#type::Methods;
use crate::methods::constant::{GET, POST};
use std::fmt::{self, Display};

impl Default for Methods {
    fn default() -> Self {
        Self::GET
    }
}

/// Provides utility methods for the `Methods` type.
impl Methods {
    /// Creates a new instance of `Methods` with the default value of `Self::GET`.
    ///
    /// This is a shorthand for using the `default` method.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the current method is `GET`.
    ///
    /// Returns `true` if the method is `GET`, otherwise returns `false`.
    pub fn is_get(&self) -> bool {
        self.to_owned() == Self::GET.to_owned()
    }

    /// Checks if the current method is `POST`.
    ///
    /// Returns `true` if the method is `POST`, otherwise returns `false`.
    pub fn is_post(&self) -> bool {
        self.to_owned() == Self::POST.to_owned()
    }
}

impl Display for Methods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::GET => GET,
            Self::POST => POST,
        };
        write!(f, "{}", res)
    }
}
