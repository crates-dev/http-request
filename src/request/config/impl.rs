use crate::*;

/// Provides default configuration values for HTTP requests.
///
/// # Returns
///
/// - `Config` - A configuration instance with default values:
///   - timeout: DEFAULT_TIMEOUT
///   - redirect: false
///   - max_redirect_times: DEFAULT_MAX_REDIRECT_TIMES
///   - buffer: DEFAULT_BUFFER_SIZE
///   - decode: true
impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            url_obj: HttpUrlComponents::default(),
            redirect: false,
            max_redirect_times: DEFAULT_MAX_REDIRECT_TIMES,
            redirect_times: 0,
            http_version: HttpVersion::default(),
            buffer: DEFAULT_BUFFER_SIZE,
            decode: true,
            proxy: None,
        }
    }
}
