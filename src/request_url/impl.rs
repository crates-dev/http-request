use super::r#type::RequestUrl;
use crate::request::error::Error;
use url::Url as UrlParser;

/// Default implementation for `RequestUrl`.
///
/// This implementation provides the default values for a `RequestUrl` instance. All fields
/// are initialized to `None` to indicate that no URL components are set by default.
///
/// # Returns
/// Returns a new `RequestUrl` instance with all fields set to `None`.
impl Default for RequestUrl {
    fn default() -> Self {
        RequestUrl {
            scheme: None,
            username: None,
            password: None,
            host: None,
            port: None,
            path: None,
            query: None,
            fragment: None,
        }
    }
}

impl RequestUrl {
    /// Parses a URL string into a `RequestUrl` instance.
    ///
    /// This method attempts to parse a given URL string into its components such as
    /// scheme, username, password, host, port, path, query, and fragment. If the URL
    /// is invalid, it returns an `Error::InvalidUrl` error.
    ///
    /// # Arguments
    /// - `url_str`: A string slice representing the URL to be parsed.
    ///
    /// # Returns
    /// Returns a `Result` containing either a `RequestUrl` instance populated with the
    /// parsed components or an `Error::InvalidUrl` if the parsing fails.
    pub fn parse(url_str: &str) -> Result<Self, Error> {
        if let Ok(parsed_url) = UrlParser::parse(url_str) {
            let res: RequestUrl = RequestUrl {
                scheme: Some(parsed_url.scheme().to_string()),
                username: if parsed_url.username().is_empty() {
                    None
                } else {
                    Some(parsed_url.username().to_string())
                },
                password: parsed_url.password().map(|p| p.to_string()),
                host: parsed_url.host_str().map(|h| h.to_string()),
                port: parsed_url.port(),
                path: Some(parsed_url.path().to_string()),
                query: parsed_url.query().map(|q| q.to_string()),
                fragment: parsed_url.fragment().map(|f| f.to_string()),
            };
            Ok(res)
        } else {
            Err(Error::InvalidUrl)
        }
    }
}
