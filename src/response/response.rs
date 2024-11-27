use std::collections::HashMap;

use super::r#type::HttpResponse;

impl HttpResponse {
    pub fn new() -> Self {
        HttpResponse {
            http_version: "HTTP/1.1".to_string(),
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn set_status_code(mut self, code: u16, text: &str) -> Self {
        self.status_code = code;
        self.status_text = text.to_string();
        self
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn set_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }
}
