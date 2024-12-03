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
/// - `RedirectInvalidUrl`: Occurs when a redirect URL is invalid.
/// - `NeedOpenRedirect`: A URL need open redirect
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
    RedirectInvalidUrl,
    NeedOpenRedirect,
}
