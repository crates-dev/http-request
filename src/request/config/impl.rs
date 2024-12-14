use super::r#type::Config;
use crate::{
    constant::request::DEFAULT_TIMEOUT, http_url::r#type::HttpUrl,
    http_version::r#type::HttpVersion,
};

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            url_obj: HttpUrl::default(),
            redirect: false,
            max_redirect_times: 8,
            redirect_times: 0,
            http_version: HttpVersion::default(),
            buffer: 1024,
            decode: true,
        }
    }
}
