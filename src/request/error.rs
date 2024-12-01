use std::{
    error::Error as StdError,
    fmt::{self},
};

/// Represents different types of errors that can occur in the application.
///
/// The `Error` enum defines various error types related to HTTP requests, network connections, and TLS operations.
/// Each variant corresponds to a specific error that can occur during the execution of the application.
///
/// # Variants
/// - `InvalidUrl`: Indicates that the provided URL is invalid.
/// - `TcpStreamConnectError`: Represents an error that occurred while attempting to connect a TCP stream.
/// - `RequestError`: A general error related to making a request.
/// - `MethodsNotSupport`: Indicates that the requested HTTP method is not supported.
/// - `ReadConnectionError`: An error that occurred while reading from the connection.
/// - `TlsConnectorBuildError`: Indicates an error during the construction of the TLS connector.
/// - `SetReadTimeoutError`: Occurs when setting the read timeout fails.
/// - `TlsStreamConnectError`: Represents an error that occurred while establishing a TLS stream connection.
/// - `MaxRedirectTimes`: Occurs when the maximum number of redirects is exceeded.
/// - `RedirectUrlDeadLoop`: Indicates that a redirect URL has resulted in a dead loop.
#[derive(Debug)]
pub enum Error {
    InvalidUrl,
    TcpStreamConnectError,
    RequestError,
    MethodsNotSupport,
    ReadConnectionError,
    TlsConnectorBuildError,
    SetReadTimeoutError,
    SetWriteTimeoutError,
    TlsStreamConnectError,
    MaxRedirectTimes,
    RedirectUrlDeadLoop,
}

impl StdError for Error {}

impl fmt::Display for Error {
    /// Formats the `Error` enum into a human-readable string.
    ///
    /// This method implements the `fmt::Display` trait for the `Error` enum, allowing it to be
    /// formatted into a string representation. Each variant is matched and a corresponding
    /// error message is returned for display.
    ///
    /// # Parameters
    /// - `f`: A mutable reference to the `fmt::Formatter` that handles the formatting of the error.
    ///
    /// # Returns
    /// A `fmt::Result` which indicates whether the formatting was successful.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidUrl => write!(f, "Invalid URL"),
            Error::TcpStreamConnectError => write!(f, "TCP Stream Connection Error"),
            Error::RequestError => write!(f, "Request Error"),
            Error::MethodsNotSupport => write!(f, "Unsupported HTTP Method"),
            Error::ReadConnectionError => write!(f, "Connection Read Error"),
            Error::TlsConnectorBuildError => write!(f, "TLS Connector Build Error"),
            Error::SetReadTimeoutError => write!(f, "Failed to Set Read Timeout"),
            Error::SetWriteTimeoutError => write!(f, "Failed to Set Write Timeout"),
            Error::TlsStreamConnectError => write!(f, "TLS Stream Connection Error"),
            Error::MaxRedirectTimes => write!(f, "Max Redirect Times"),
            Error::RedirectUrlDeadLoop => write!(f, "Redirect URL Dead Loop"),
        }
    }
}
