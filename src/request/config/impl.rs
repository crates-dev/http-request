use super::r#type::Config;
use http_type::*;

impl Default for Config {
    #[inline]
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
        }
    }
}
