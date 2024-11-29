/// Defines the `Methods` enum, representing HTTP request methods.
///
/// The `Methods` enum includes commonly used HTTP methods such as `GET` and `POST`.
/// It derives the following traits:
/// - `Debug`: Enables formatting the enum with the `{:?}` formatter for debugging purposes.
/// - `Clone`: Allows creating a duplicate of a `Methods` value.
/// - `PartialEq`: Enables equality comparison between `Methods` values.
#[derive(Debug, Clone, PartialEq)]
pub enum Methods {
    /// Represents the HTTP `GET` method.
    GET,
    /// Represents the HTTP `POST` method.
    POST,
}
