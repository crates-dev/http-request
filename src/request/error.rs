use std::{
    error::Error as StdError,
    fmt::{self},
};

/// Custom error type for handling various HTTP-related errors.
///
/// This `Error` enum is designed to capture and represent specific errors that can occur
/// during HTTP operations, including issues with URL parsing, TCP stream connections,
/// request processing, unsupported HTTP methods, and connection read failures.
/// It provides clear and structured error reporting for HTTP-related functionalities.
///
/// # Variants
///
/// - `InvalidUrl`:
///   Indicates that the provided URL is invalid, such as malformed or missing critical components.
/// - `TcpStreamConnectError`:
///   Represents a failure while attempting to establish a TCP stream connection.
/// - `RequestError`:
///   A general error that occurs during the processing of an HTTP request.
/// - `MethodsNotSupport`:
///   Signifies that the specified HTTP method is unsupported by the library or server.
/// - `ReadConnectionError`:
///   Occurs when reading data from a connection fails, such as during a response retrieval.
/// - `TlsConnectorBuildError`:
///   Indicates an error while constructing a TLS connector, potentially due to configuration issues.
/// - `SetReadTimeoutError`:
///   Represents an error when setting the read timeout on a connection fails.
/// - `TlsStreamConnectError`:
///   Occurs when a TLS-secured connection cannot be established.
///
/// # Traits Implemented
///
/// - `StdError`:
///   Enables integration with Rust's standard error handling mechanisms, such as `Result`.
/// - `fmt::Display`:
///   Provides human-readable error messages for debugging or logging.
#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
    MethodsNotSupport,
    ReadConnectionError,
    TlsConnectorBuildError,
    SetReadTimeoutError,
    TlsStreamConnectError,
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
            Error::TcpStreamConnectError => write!(f, "TCP Stream Connection Error"),
            Error::RequestError => write!(f, "Request Error"),
            Error::MethodsNotSupport => write!(f, "Unsupported HTTP Method"),
            Error::ReadConnectionError => write!(f, "Connection Read Error"),
            Error::TlsConnectorBuildError => write!(f, "TLS Connector Build Error"),
            Error::SetReadTimeoutError => write!(f, "Failed to Set Read Timeout"),
            Error::TlsStreamConnectError => write!(f, "TLS Stream Connection Error"),
        }
    }
}
