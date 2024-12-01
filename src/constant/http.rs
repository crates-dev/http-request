/// A carriage return and newline character sequence (`\r\n`), used to separate lines in HTTP headers.
pub static HTTP_BR: &str = "\r\n";

/// A static reference to a byte slice representing the HTTP line break (`\r\n`).
pub static HTTP_BR_BYTES: &[u8] = HTTP_BR.as_bytes();

/// A double carriage return and newline character sequence (`\r\n\r\n`), used to separate HTTP headers from the body.
pub static HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// A static reference to a byte slice representing the HTTP double line break (`\r\n\r\n`).
pub static HTTP_DOUBLE_BR_BYTES: &[u8] = HTTP_DOUBLE_BR.as_bytes();

/// The HTTP header field name `Location`, used to specify the URL to redirect a client.
pub static LOCATION: &str = "Location";

/// The HTTP header field name `Content-Length`, used to specify the length of the response body in bytes.
pub static CONTENT_LENGTH: &str = "Content-Length";

/// The HTTP header field name `Content-Type`, used to specify the media type of the resource or the data being sent in an HTTP request or response.
pub static CONTENT_TYPE: &str = "Content-Type";

/// The HTTP header field "Accept".
pub static ACCEPT: &str = "Accept";

/// The default value for the `Accept` header.
pub static ACCEPT_VALUE: &str = "*/*";

/// The HTTP header field "User-Agent".
pub static USER_AGENT: &str = "User-Agent";

/// The HTTP header field name `Host`, used to specify the host and port number of the server.
pub static HOST: &str = "Host";

/// Unknown HTTP version
pub static UNKNOWN_HTTP_VERSION: &str = "";

/// The default HTTP version `HTTP/1.1` used in requests and responses.
pub static HTTP_VERSION_1_1: &str = "HTTP/1.1";

/// The default HTTP version `HTTP/2` used in requests and responses.
pub static HTTP_VERSION_2: &str = "HTTP/2";

/// The default HTTP path (`/`), typically used in requests when no specific path is provided.
pub static DEFAULT_HTTP_PATH: &str = "/";

/// The MIME type for JSON content, typically used for requests and responses
/// containing JSON data.
pub static APPLICATION_JSON: &str = "application/json";

/// The MIME type for XML content, typically used for requests and responses
/// containing XML data.
pub static APPLICATION_XML: &str = "application/xml";

/// The MIME type for plain text content, typically used for requests and responses
/// containing simple text data.
pub static TEXT_PLAIN: &str = "text/plain";

/// The MIME type for HTML content, typically used for requests and responses
/// containing HTML data.
pub static TEXT_HTML: &str = "text/html";

/// The MIME type for form-encoded data, commonly used for sending data in the
/// body of HTTP requests, especially for form submissions.
pub static FORM_URLENCODED: &str = "application/x-www-form-urlencoded";

/// Query symbols
pub static QUERY_SYMBOL: &str = "?";
