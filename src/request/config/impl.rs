use super::r#type::Config;
use crate::{constant::request::DEFAULT_TIMEOUT, request::request_url::r#type::RequestUrl};

impl Default for Config {
    fn default() -> Self {
        Config {
            timeout: DEFAULT_TIMEOUT,
            url_obj: RequestUrl::default(),
            redirect: false,
            max_redirect_times: 8,
            redirect_times: 0,
        }
    }
}
