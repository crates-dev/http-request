use super::r#type::HttpResponseText;
use crate::response::{r#trait::ResponseTrait, response_binary::r#type::HttpResponseBinary};
use http_compress::Compress;
use http_type::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Implements the `ResponseTrait` trait for `HttpResponseText`.
///
/// This implementation allows `HttpResponseText` to convert between text and binary
/// representations of HTTP responses. It provides methods for parsing raw responses, as well
/// as accessing text and binary formats.
///
/// # Associated Types
/// - `OutputText`: Specifies the text representation of an HTTP response (`HttpResponseText`).
/// - `OutputBinary`: Specifies the binary representation of an HTTP response (`HttpResponseBinary`).
impl ResponseTrait for HttpResponseText {
    type OutputText = HttpResponseText;
    type OutputBinary = HttpResponseBinary;

    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <HttpResponseBinary as ResponseTrait>::from(response).text()
    }

    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    fn binary(&self) -> HttpResponseBinary {
        let body: Vec<u8> = self
            .body
            .read()
            .map_or(Vec::new(), |body| body.clone().into_bytes());
        HttpResponseBinary {
            http_version: self.http_version.clone(),
            status_code: self.status_code,
            status_text: self.status_text.clone(),
            headers: self.headers.clone(),
            body: Arc::new(RwLock::new(body)),
        }
    }

    fn decode(&self, buffer_size: usize) -> HttpResponseBinary {
        let http_response: HttpResponseText = self.clone();
        let tmp_body: Vec<u8> = self
            .body
            .read()
            .map_or(Vec::new(), |body| body.as_bytes().to_vec())
            .to_vec();
        let body: Vec<u8> = Compress::from(
            &self
                .headers
                .read()
                .map_or(HashMap::new(), |headers| headers.clone()),
        )
        .decode(&tmp_body, buffer_size);
        HttpResponseBinary {
            http_version: http_response.http_version,
            status_code: http_response.status_code,
            status_text: http_response.status_text,
            headers: http_response.headers,
            body: Arc::new(RwLock::new(body)),
        }
    }
}

impl Default for HttpResponseText {
    fn default() -> Self {
        Self {
            http_version: Arc::new(RwLock::new(HttpVersion::Unknown(String::new()).to_string())),
            status_code: StatusCode::Unknown.code(),
            status_text: Arc::new(RwLock::new(StatusCode::Unknown.to_string())),
            headers: Arc::new(RwLock::new(HashMap::new())),
            body: Arc::new(RwLock::new(String::new())),
        }
    }
}
