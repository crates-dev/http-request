use crate::*;

/// HTTP request error types.
///
/// Comprehensive error handling for HTTP requests including network, protocol,
/// and application-level errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestError {
    /// Unknown error with HTTP status
    Unknown(HttpStatus),
    /// HTTP read error with HTTP status
    HttpRead(HttpStatus),
    /// TCP stream connection error with HTTP status
    GetTcpStream(HttpStatus),
    /// TLS stream connection error with HTTP status
    GetTlsStream(HttpStatus),
    /// Connection read error with HTTP status
    ReadConnection(HttpStatus),
    /// Request was aborted with HTTP status
    RequestAborted(HttpStatus),
    /// TLS stream connection failed with HTTP status
    TlsStreamConnect(HttpStatus),
    /// Redirect functionality needs to be enabled with HTTP status
    NeedOpenRedirect(HttpStatus),
    /// Maximum redirect times exceeded with HTTP status
    MaxRedirectTimes(HttpStatus),
    /// HTTP method not supported with HTTP status
    MethodsNotSupport(HttpStatus),
    /// Redirect URL is invalid with HTTP status
    RedirectInvalidUrl(HttpStatus),
    /// Client disconnected with HTTP status
    ClientDisconnected(HttpStatus),
    /// Redirect URL dead loop detected with HTTP status
    RedirectUrlDeadLoop(HttpStatus),
    /// Client closed connection with HTTP status
    ClientClosedConnection(HttpStatus),
    /// Incomplete WebSocket frame with HTTP status
    IncompleteWebSocketFrame(HttpStatus),
    /// Request too long with HTTP status
    RequestTooLong(HttpStatus),
    /// Path too long with HTTP status
    PathTooLong(HttpStatus),
    /// Query too long with HTTP status
    QueryTooLong(HttpStatus),
    /// Header line too long with HTTP status
    HeaderLineTooLong(HttpStatus),
    /// Too many headers with HTTP status
    TooManyHeaders(HttpStatus),
    /// Header key too long with HTTP status
    HeaderKeyTooLong(HttpStatus),
    /// Header value too long with HTTP status
    HeaderValueTooLong(HttpStatus),
    /// Content length too large with HTTP status
    ContentLengthTooLarge(HttpStatus),
    /// Invalid content length with HTTP status
    InvalidContentLength(HttpStatus),
    /// Invalid URL scheme with HTTP status
    InvalidUrlScheme(HttpStatus),
    /// Invalid URL host with HTTP status
    InvalidUrlHost(HttpStatus),
    /// Invalid URL port with HTTP status
    InvalidUrlPort(HttpStatus),
    /// Invalid URL path with HTTP status
    InvalidUrlPath(HttpStatus),
    /// Invalid URL query with HTTP status
    InvalidUrlQuery(HttpStatus),
    /// Invalid URL fragment with HTTP status
    InvalidUrlFragment(HttpStatus),
    /// Read timeout not set with HTTP status
    ReadTimeoutNotSet(HttpStatus),
    /// Write timeout not set with HTTP status
    WriteTimeoutNotSet(HttpStatus),
    /// TCP connection failed with HTTP status
    TcpConnectionFailed(HttpStatus),
    /// TLS handshake failed with HTTP status
    TlsHandshakeFailed(HttpStatus),
    /// TLS certificate invalid with HTTP status
    TlsCertificateInvalid(HttpStatus),
    /// WebSocket frame too large with HTTP status
    WebSocketFrameTooLarge(HttpStatus),
    /// WebSocket opcode unsupported with HTTP status
    WebSocketOpcodeUnsupported(HttpStatus),
    /// WebSocket mask missing with HTTP status
    WebSocketMaskMissing(HttpStatus),
    /// WebSocket payload corrupted with HTTP status
    WebSocketPayloadCorrupted(HttpStatus),
    /// WebSocket invalid UTF-8 with HTTP status
    WebSocketInvalidUtf8(HttpStatus),
    /// WebSocket invalid close code with HTTP status
    WebSocketInvalidCloseCode(HttpStatus),
    /// WebSocket invalid extension with HTTP status
    WebSocketInvalidExtension(HttpStatus),
    /// HTTP request parts insufficient with HTTP status
    HttpRequestPartsInsufficient(HttpStatus),
    /// TCP stream connection error with HTTP status  
    TcpStreamConnect(HttpStatus),
    /// TLS connector build error with HTTP status
    TlsConnectorBuild(HttpStatus),
    /// Invalid URL error with HTTP status
    InvalidUrl(HttpStatus),
    /// Set read timeout error with HTTP status
    SetReadTimeout(HttpStatus),
    /// Set write timeout error with HTTP status
    SetWriteTimeout(HttpStatus),
    /// Generic request error with string message
    Request(String),
    /// TCP stream connection error with string message
    TcpStreamConnectString(String),
    /// TLS connector build error with string message
    TlsConnectorBuildString(String),
    /// Set read timeout error with string message
    SetReadTimeoutString(String),
    /// Set write timeout error with string message
    SetWriteTimeoutString(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::Unknown(status) => write!(f, "Unknown error: {status:?}"),
            RequestError::HttpRead(status) => write!(f, "HTTP read error: {status:?}"),
            RequestError::GetTcpStream(status) => write!(f, "TCP stream error: {status:?}"),
            RequestError::GetTlsStream(status) => write!(f, "TLS stream error: {status:?}"),
            RequestError::ReadConnection(status) => {
                write!(f, "Connection read error: {status:?}")
            }
            RequestError::RequestAborted(status) => write!(f, "Request aborted: {status:?}"),
            RequestError::TlsStreamConnect(status) => {
                write!(f, "TLS stream connect error: {status:?}")
            }
            RequestError::NeedOpenRedirect(status) => {
                write!(f, "Redirect not enabled: {status:?}")
            }
            RequestError::MaxRedirectTimes(status) => {
                write!(f, "Max redirects exceeded: {status:?}")
            }
            RequestError::MethodsNotSupport(status) => {
                write!(f, "Method not supported: {status:?}")
            }
            RequestError::RedirectInvalidUrl(status) => {
                write!(f, "Invalid redirect URL: {status:?}")
            }
            RequestError::ClientDisconnected(status) => {
                write!(f, "Client disconnected: {status:?}")
            }
            RequestError::RedirectUrlDeadLoop(status) => {
                write!(f, "Redirect loop detected: {status:?}")
            }
            RequestError::ClientClosedConnection(status) => {
                write!(f, "Client closed connection: {status:?}")
            }
            RequestError::IncompleteWebSocketFrame(status) => {
                write!(f, "Incomplete WebSocket frame: {status:?}")
            }
            RequestError::RequestTooLong(status) => write!(f, "Request too long: {status:?}"),
            RequestError::PathTooLong(status) => write!(f, "Path too long: {status:?}"),
            RequestError::QueryTooLong(status) => write!(f, "Query too long: {status:?}"),
            RequestError::HeaderLineTooLong(status) => {
                write!(f, "Header line too long: {status:?}")
            }
            RequestError::TooManyHeaders(status) => write!(f, "Too many headers: {status:?}"),
            RequestError::HeaderKeyTooLong(status) => {
                write!(f, "Header key too long: {status:?}")
            }
            RequestError::HeaderValueTooLong(status) => {
                write!(f, "Header value too long: {status:?}")
            }
            RequestError::ContentLengthTooLarge(status) => {
                write!(f, "Content length too large: {status:?}")
            }
            RequestError::InvalidContentLength(status) => {
                write!(f, "Invalid content length: {status:?}")
            }
            RequestError::InvalidUrlScheme(status) => write!(f, "Invalid URL scheme: {status:?}"),
            RequestError::InvalidUrlHost(status) => write!(f, "Invalid URL host: {status:?}"),
            RequestError::InvalidUrlPort(status) => write!(f, "Invalid URL port: {status:?}"),
            RequestError::InvalidUrlPath(status) => write!(f, "Invalid URL path: {status:?}"),
            RequestError::InvalidUrlQuery(status) => write!(f, "Invalid URL query: {status:?}"),
            RequestError::InvalidUrlFragment(status) => {
                write!(f, "Invalid URL fragment: {status:?}")
            }
            RequestError::ReadTimeoutNotSet(status) => {
                write!(f, "Read timeout not set: {status:?}")
            }
            RequestError::WriteTimeoutNotSet(status) => {
                write!(f, "Write timeout not set: {status:?}")
            }
            RequestError::TcpConnectionFailed(status) => {
                write!(f, "TCP connection failed: {status:?}")
            }
            RequestError::TlsHandshakeFailed(status) => {
                write!(f, "TLS handshake failed: {status:?}")
            }
            RequestError::TlsCertificateInvalid(status) => {
                write!(f, "TLS certificate invalid: {status:?}")
            }
            RequestError::WebSocketFrameTooLarge(status) => {
                write!(f, "WebSocket frame too large: {status:?}")
            }
            RequestError::WebSocketOpcodeUnsupported(status) => {
                write!(f, "WebSocket opcode unsupported: {status:?}")
            }
            RequestError::WebSocketMaskMissing(status) => {
                write!(f, "WebSocket mask missing: {status:?}")
            }
            RequestError::WebSocketPayloadCorrupted(status) => {
                write!(f, "WebSocket payload corrupted: {status:?}")
            }
            RequestError::WebSocketInvalidUtf8(status) => {
                write!(f, "WebSocket invalid UTF-8: {status:?}")
            }
            RequestError::WebSocketInvalidCloseCode(status) => {
                write!(f, "WebSocket invalid close code: {status:?}")
            }
            RequestError::WebSocketInvalidExtension(status) => {
                write!(f, "WebSocket invalid extension: {status:?}")
            }
            RequestError::HttpRequestPartsInsufficient(status) => {
                write!(f, "HTTP request parts insufficient: {status:?}")
            }
            RequestError::TcpStreamConnect(status) => {
                write!(f, "TCP stream connect error: {status:?}")
            }
            RequestError::TlsConnectorBuild(status) => {
                write!(f, "TLS connector build error: {status:?}")
            }
            RequestError::InvalidUrl(status) => write!(f, "Invalid URL: {status:?}"),
            RequestError::SetReadTimeout(status) => {
                write!(f, "Set read timeout error: {status:?}")
            }
            RequestError::SetWriteTimeout(status) => {
                write!(f, "Set write timeout error: {status:?}")
            }
            RequestError::Request(msg) => write!(f, "Request error: {msg}"),
            RequestError::TcpStreamConnectString(msg) => {
                write!(f, "TCP stream connect error: {msg}")
            }
            RequestError::TlsConnectorBuildString(msg) => {
                write!(f, "TLS connector build error: {msg}")
            }
            RequestError::SetReadTimeoutString(msg) => write!(f, "Set read timeout error: {msg}"),
            RequestError::SetWriteTimeoutString(msg) => {
                write!(f, "Set write timeout error: {msg}")
            }
        }
    }
}
