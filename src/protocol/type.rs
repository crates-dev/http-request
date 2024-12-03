/// Defines the `Protocol` enum, representing HTTP-related protocols.
///
/// The `Protocol` enum includes:
/// - `HTTP`: Represents the HTTP protocol.
/// - `HTTPS`: Represents the HTTPS protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    /// Represents the HTTP protocol.
    HTTP,
    /// Represents the HTTPS protocol.
    HTTPS,
    /// Unknown
    Unknown(String),
}
