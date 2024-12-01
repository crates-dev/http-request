use super::r#type::Tmp;
use std::collections::HashSet;

impl Default for Tmp {
    /// Provides the default value for `Tmp`.
    ///
    /// This method initializes a `Tmp` instance with default values. Specifically, it sets
    /// `visit_url` to an empty `HashSet`, which is used to store visited URLs. This can be
    /// useful for tracking URLs that have already been visited in the context of HTTP requests
    /// or other network-related operations.
    ///
    /// # Returns
    /// Returns a `Tmp` instance with a default state, where:
    /// - `visit_url` is an empty `HashSet` of URLs.
    fn default() -> Self {
        Tmp {
            visit_url: HashSet::new(),
        }
    }
}
