/// Defines the `Protocol` enum, representing HTTP-related protocols.
///
/// The `Protocol` enum includes:
/// - `HTTP`: Represents the HTTP protocol.
/// - `HTTPS`: Represents the HTTPS protocol.
///
/// It derives the following traits:
/// - `Debug`: Enables formatting the enum with the `{:?}` formatter for debugging purposes.
/// - `Clone`: Allows creating a duplicate of a `Protocol` value.
/// - `PartialEq`: Enables equality comparison between `Protocol` values.
#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    /// Represents the HTTP protocol.
    HTTP,
    /// Represents the HTTPS protocol.
    HTTPS,
}
