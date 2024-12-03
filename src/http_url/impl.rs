use super::r#type::HttpUrl;
use crate::{protocol::r#type::Protocol, request::error::Error};
use url::Url as UrlParser;

/// Default implementation for `HttpUrl`.
///
/// This implementation provides the default values for a `HttpUrl` instance. All fields
/// are initialized to `None` to indicate that no URL components are set by default.
///
/// # Returns
/// Returns a new `HttpUrl` instance with all fields set to `None`.
impl Default for HttpUrl {
    fn default() -> Self {
        HttpUrl {
            protocol: Protocol::Unknown(String::new()),
            host: None,
            port: None,
            path: None,
            query: None,
            fragment: None,
        }
    }
}

impl HttpUrl {
    /// Parses a URL string into a `HttpUrl` instance.
    ///
    /// This method attempts to parse a given URL string into its components such as
    /// scheme, username, password, host, port, path, query, and fragment. If the URL
    /// is invalid, it returns an `Error::InvalidUrl` error.
    ///
    /// # Parameters
    /// - `url_str`: A string slice representing the URL to be parsed.
    ///
    /// # Returns
    /// Returns a `Result` containing either a `HttpUrl` instance populated with the
    /// parsed components or an `Error::InvalidUrl` if the parsing fails.
    pub fn parse(url_str: &str) -> Result<Self, Error> {
        if let Ok(parsed_url) = UrlParser::parse(url_str) {
            let res: HttpUrl = HttpUrl {
                protocol: parsed_url
                    .scheme()
                    .to_string()
                    .parse::<Protocol>()
                    .unwrap_or_default(),
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
