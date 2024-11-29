use super::r#type::HttpResponse;
use crate::{
    constant::http::{CONTENT_LENGTH, DEFAULT_HTTP_VERSION, HTTP_BR},
    status_code::r#type::StatusCode,
    *,
};
use std::str::Lines;

impl HttpResponse {
    pub fn get_content_length(response_string: &str) -> usize {
        let content_length_sign_key: String = format!("{}:", CONTENT_LENGTH.to_lowercase());
        response_string
            .to_lowercase()
            .find(&content_length_sign_key)
            .and_then(|length_pos| {
                let start: usize = length_pos + content_length_sign_key.len();
                let tmp_res = response_string[start..]
                    .find(HTTP_BR)
                    .and_then(|end| {
                        let content_length: usize = response_string[start..start + end]
                            .trim()
                            .parse()
                            .unwrap_or(0);
                        Some(content_length)
                    })
                    .unwrap_or_default();
                Some(tmp_res)
            })
            .unwrap_or_default()
    }

    pub fn from(response: &str) -> Self {
        let mut lines: Lines<'_> = response.lines();
        let status_line: &str = lines.next().unwrap_or("");
        let status_parts: Vec<&str> = status_line.split_whitespace().collect();
        let http_version: String = status_parts
            .get(0)
            .unwrap_or(&DEFAULT_HTTP_VERSION)
            .to_string();
        let status_code: u16 = status_parts
            .get(1)
            .unwrap_or(&StatusCode::Ok.to_string().as_str())
            .parse()
            .unwrap_or(StatusCode::Unknown.code());
        let status_text: String = status_parts.get(2..).unwrap_or(&[]).join(" ");
        let mut headers: HashMap<String, String> = HashMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
            if header_parts.len() == 2 {
                let key: String = header_parts[0].trim().to_string();
                let value: String = header_parts[1].trim().to_string();
                headers.insert(key, value);
            }
        }
        let body: String = lines.collect::<Vec<&str>>().join("\n");
        HttpResponse {
            http_version,
            status_code,
            status_text,
            headers,
            body,
        }
    }
}

impl Default for HttpResponse {
    fn default() -> Self {
        HttpResponse {
            http_version: DEFAULT_HTTP_VERSION.to_string(),
            status_code: StatusCode::Unknown.code(),
            status_text: StatusCode::Unknown.to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}
