use super::r#type::Config;
use crate::{
    constant::request::DEFAULT_TIMEOUT,
    request::{http_version::r#type::HttpVersion, request_url::r#type::RequestUrl},
};

impl Default for Config {
    /// Provides the default configuration for `Config`.
    ///
    /// This method initializes a `Config` instance with default values:
    /// - `timeout`: Set to the constant `DEFAULT_TIMEOUT`.
    /// - `url_obj`: Initialized with the default value of `RequestUrl`.
    /// - `redirect`: Set to `false` to disable redirects by default.
    /// - `max_redirect_times`: Set to `8` to limit the number of allowed redirects.
    /// - `redirect_times`: Set to `0` indicating no redirects have been made.
    /// - `http_version`: Set to the default value of `HttpVersion`.
    ///
    /// # Returns
    /// Returns a `Config` instance with the default settings.
    fn default() -> Self {
        Config {
            timeout: DEFAULT_TIMEOUT,
            url_obj: RequestUrl::default(),
            redirect: false,
            max_redirect_times: 8,
            redirect_times: 0,
            http_version: HttpVersion::default(),
        }
    }
}
