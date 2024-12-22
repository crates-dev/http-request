use super::r#type::Config;
use crate::constant::r#type::DEFAULT_TIMEOUT;
use http_type::*;

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            url_obj: HttpUrlComponents::default(),
            redirect: false,
            max_redirect_times: 8,
            redirect_times: 0,
            http_version: HttpVersion::default(),
            buffer: 1024,
            decode: true,
        }
    }
}
