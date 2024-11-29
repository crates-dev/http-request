use super::r#type::HttpRequestBuilder;
use crate::{
    global_type::r#type::{Body, Header},
    request::http_request::r#type::HttpRequest,
    Methods, *,
};

impl Default for HttpRequestBuilder {
    fn default() -> HttpRequestBuilder {
        HttpRequestBuilder {
            tmp: HttpRequest::default(),
            builder: HttpRequest::default(),
        }
    }
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        HttpRequestBuilder::default()
    }

    pub fn set_methods(&mut self, methods: Methods) -> &mut Self {
        self.tmp.methods = Arc::new(methods);
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.tmp.url = Arc::new(url.to_owned());
        self
    }

    pub fn set_header(&mut self, header: &Header) -> &mut Self {
        if let Some(tmp_header) = Arc::get_mut(&mut self.tmp.header) {
            for (key, value) in header {
                tmp_header.insert(key.clone(), value.clone());
            }
        }
        self
    }

    pub fn set_body(&mut self, body: &Body) -> &mut Self {
        if let Some(tmp_body) = Arc::get_mut(&mut self.tmp.body) {
            for (key, value) in body {
                tmp_body.insert(key.clone(), value.clone());
            }
        }
        self
    }

    pub fn builder(&mut self) -> HttpRequest {
        self.builder = self.tmp.clone();
        self.tmp = HttpRequest::default();
        self.builder.clone()
    }
}
