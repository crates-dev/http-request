/// Defines the `Methods` enum, representing HTTP request methods.
///
/// The `Methods` enum includes commonly used HTTP methods such as `GET` and `POST`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Methods {
    /// Represents the HTTP `GET` method.
    GET,
    /// Represents the HTTP `POST` method.
    POST,
}
