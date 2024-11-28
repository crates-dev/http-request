use super::error::Error;
use super::r#type::RequestUrl;
use url::Url as UrlParser;

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
