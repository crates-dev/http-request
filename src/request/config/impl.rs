use super::r#type::Config;
use crate::{
    constant::request::DEFAULT_TIMEOUT, http_url::r#type::HttpUrl,
    http_version::r#type::HttpVersion,
};

impl Default for Config {
    /// Provides the default configuration for `Config`.
    ///
    /// This method initializes a `Config` instance with default values:
    /// - `timeout`: Set to the constant `DEFAULT_TIMEOUT`, which represents the default timeout duration.
    /// - `url_obj`: Initialized with the default value of `HttpUrl`. This represents the URL used in the request.
    /// - `redirect`: Set to `false` by default, meaning that HTTP redirects are disabled unless explicitly enabled.
    /// - `max_redirect_times`: Set to `8` by default, which limits the maximum number of redirects that can be followed.
    /// - `redirect_times`: Set to `0`, indicating that no redirects have been followed initially.
    /// - `http_version`: Set to the default value of `HttpVersion`, which defines the default HTTP protocol version used in the request.
    /// - `buffer`: Set to `1024`, defining the default buffer size (in bytes) for reading the HTTP response.
    ///
    /// # Returns
    ///
    /// Returns a `Config` instance initialized with the default settings as described above.
    /// The default configuration is suitable for most typical HTTP requests, but can be customized
    /// as needed by modifying individual fields after the instance is created.
    fn default() -> Self {
        Config {
            timeout: DEFAULT_TIMEOUT,
            url_obj: HttpUrl::default(),
            redirect: false,
            max_redirect_times: 8,
            redirect_times: 0,
            http_version: HttpVersion::default(),
            buffer: 1024,
        }
    }
}
