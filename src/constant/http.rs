/// A carriage return and newline character sequence (`\r\n`), used to separate lines in HTTP headers.
pub static HTTP_BR: &str = "\r\n";

/// A double carriage return and newline character sequence (`\r\n\r\n`), used to separate HTTP headers from the body.
pub static HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// The HTTP header field name `Location`, used to specify the URL to redirect a client.
pub static LOCATION: &str = "Location";

/// The HTTP header field name `Content-Length`, used to specify the length of the response body in bytes.
pub static CONTENT_LENGTH: &str = "Content-Length";

/// The HTTP header field name `Content-Type`, used to specify the media type of the resource or the data being sent in an HTTP request or response.
pub static CONTENT_TYPE: &str = "Content-Type";

/// The HTTP header field name `Host`, used to specify the host and port number of the server.
pub static HOST: &str = "Host";

/// The default HTTP version `HTTP/1.1` used in requests and responses.
pub static DEFAULT_HTTP_VERSION: &str = "HTTP/1.1";

/// The default HTTP path (`/`), typically used in requests when no specific path is provided.
pub static DEFAULT_HTTP_PATH: &str = "/";

/// The HTTP header field name `Connection`, used to specify control options for the current connection.
pub static CONNECTION: &str = "Connection";

/// The MIME type `application/json`, used to indicate that the data being sent or received is in JSON (JavaScript Object Notation) format.
pub static APPLICATION_JSON: &str = "application/json";
pub static APPLICATION_XML: &str = "application/xml";
pub static TEXT_PLAIN: &str = "text/plain";
pub static TEXT_HTML: &str = "text/html";
pub static FORM_URLENCODED: &str = "application/x-www-form-urlencoded";
