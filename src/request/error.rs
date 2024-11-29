use std::{error::Error as StdError, fmt};

/// Custom error type for handling various HTTP-related errors.
///
/// This `Error` enum is used to represent different kinds of errors that can occur
/// in the HTTP request and connection process. It provides specific variants to
/// distinguish between various failure points, such as URL issues, connection failures,
/// and unsupported HTTP methods.
///
/// # Variants
/// - `InvalidUrl`: Represents an error when the provided URL is invalid.
/// - `TcpStreamConnectError`: Indicates an error when attempting to connect to a TCP stream.
/// - `RequestError`: General error indicating an issue with processing the HTTP request.
/// - `MethodsNotSupport`: Represents an error when the HTTP method is not supported.
/// - `ReadConnectionError`: Indicates an error when reading from the connection fails.
///
/// This custom error type implements the `StdError` trait for compatibility with
/// standard error handling mechanisms, and the `fmt::Display` trait for human-readable
/// error messages.
#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
    MethodsNotSupport,
    ReadConnectionError,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
            Error::TcpStreamConnectError => write!(f, "Tcp Stream Connect Error"),
            Error::RequestError => write!(f, "Request Error"),
            Error::MethodsNotSupport => write!(f, "Methods Not Support"),
            Error::ReadConnectionError => write!(f, "Read Connection Error"),
        }
    }
}
