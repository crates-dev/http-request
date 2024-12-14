// TODO:
use crate::protocol::r#type::Protocol;

/// A struct representing a parsed URL with various components.
///
/// This struct is used to store the different components of a URL, such as the scheme,
/// username, password, host, port, path, query, and fragment. It allows for easy
/// handling and manipulation of URL data.
///
/// # Fields
/// - `scheme`: The URL scheme (e.g., "http", "https") as a string, or `None` if not specified.
/// - `host`: The host portion of the URL (e.g., "example.com"), or `None` if not specified.
/// - `port`: The port number, if specified, or `None` if not specified.
/// - `path`: The path portion of the URL (e.g., "/path/to/resource"), or `None` if not specified.
/// - `query`: The query string, if present, or `None` if not specified.
/// - `fragment`: The fragment identifier, if present, or `None` if not specified.
///
/// This struct is primarily used for holding the components of a URL after parsing, allowing
/// for easy manipulation and access to the individual components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpUrl {
    pub protocol: Protocol,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}
