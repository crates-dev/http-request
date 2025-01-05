use super::r#type::HttpResponseBinary;
use crate::{
    response::{r#trait::ResponseTrait, response_text::r#type::HttpResponseText},
    utils::vec::{split_multi_byte, split_whitespace},
};
use http_compress::Compress;
use http_type::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    vec::IntoIter,
};

/// Implements the `ResponseTrait` trait for `HttpResponseBinary`.
///
/// This implementation specifies the associated types for binary and text representations
/// of HTTP responses, enabling seamless conversion and handling of HTTP response data.
///
/// # Associated Types
/// - `OutputText`: Specifies the text representation of an HTTP response (`HttpResponseText`).
/// - `OutputBinary`: Specifies the binary representation of an HTTP response (`HttpResponseBinary`).
impl ResponseTrait for HttpResponseBinary {
    type OutputText = HttpResponseText;
    type OutputBinary = HttpResponseBinary;

    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        let split_lines: Vec<&[u8]> = split_multi_byte(response, HTTP_BR_BYTES);
        let mut lines: IntoIter<&[u8]> = split_lines.into_iter();
        let status_line: &[u8] = lines.next().unwrap_or(&[]);
        let status_parts: Vec<&[u8]> = split_whitespace(&status_line);
        let http_version: String = String::from_utf8_lossy(
            status_parts
                .get(0)
                .unwrap_or(&HttpVersion::Unknown(String::new()).to_string().as_bytes()),
        )
        .to_string();
        let status_code: u16 = status_parts
            .get(1)
            .and_then(|part| std::str::from_utf8(part).ok())
            .unwrap_or(&StatusCode::Ok.to_string())
            .parse()
            .unwrap_or(StatusCode::Unknown.code());
        let status_text: String = status_parts.get(2..).map_or_else(
            || StatusCode::Unknown.to_string(),
            |parts| String::from_utf8_lossy(&parts.concat()).to_string(),
        );
        let mut headers: HashMap<String, String> = HashMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let header_parts: Vec<&[u8]> = split_multi_byte(&line, COLON_SPACE_BYTES);
            if header_parts.len() == 2 {
                let key: String = String::from_utf8_lossy(header_parts[0]).trim().to_string();
                let value: String = String::from_utf8_lossy(header_parts[1]).trim().to_string();
                headers.insert(key, value);
            }
        }
        let body: Vec<u8> = lines.clone().collect::<Vec<&[u8]>>().join(BR_BYTES);
        HttpResponseBinary {
            http_version: Arc::new(RwLock::new(http_version)),
            status_code,
            status_text: Arc::new(RwLock::new(status_text)),
            headers: Arc::new(RwLock::new(headers)),
            body: Arc::new(RwLock::new(body)),
        }
    }

    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    fn text(&self) -> HttpResponseText {
        let http_response: HttpResponseBinary = self.clone();
        let body_bin: Vec<u8> = http_response
            .body
            .read()
            .map_or(Vec::new(), |body| body.clone());
        let body: String = String::from_utf8_lossy(&body_bin).to_string();
        HttpResponseText {
            http_version: http_response.http_version,
            status_code: http_response.status_code,
            status_text: http_response.status_text,
            headers: http_response.headers,
            body: Arc::new(RwLock::new(body)),
        }
    }

    fn decode(&self, buffer_size: usize) -> HttpResponseBinary {
        let http_response: HttpResponseBinary = self.clone();
        let body: Vec<u8> = Compress::from(
            &self
                .headers
                .read()
                .map_or(HashMap::new(), |headers| headers.clone()),
        )
        .decode(
            &self.body.read().map_or(Vec::new(), |body| body.clone()),
            buffer_size,
        );
        HttpResponseBinary {
            http_version: http_response.http_version,
            status_code: http_response.status_code,
            status_text: http_response.status_text,
            headers: http_response.headers,
            body: Arc::new(RwLock::new(body)),
        }
    }
}

impl Default for HttpResponseBinary {
    fn default() -> Self {
        Self {
            http_version: Arc::new(RwLock::new(HttpVersion::Unknown(String::new()).to_string())),
            status_code: StatusCode::Unknown.code(),
            status_text: Arc::new(RwLock::new(StatusCode::Unknown.to_string())),
            headers: Arc::new(RwLock::new(HashMap::new())),
            body: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
