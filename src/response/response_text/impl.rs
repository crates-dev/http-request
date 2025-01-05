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

impl HttpResponseText {
    /// Retrieves the HTTP version associated with this response.
    ///
    /// # Returns
    /// - `HttpVersion`: The HTTP version (e.g., HTTP/1.1, HTTP/2, etc.) used for the response.
    pub fn get_http_version(&self) -> HttpVersion {
        if let Ok(http_version) = self.http_version.read() {
            return http_version
                .to_string()
                .parse::<HttpVersion>()
                .unwrap_or_default();
        }
        return HttpVersion::default();
    }

    /// Retrieves the HTTP status code associated with this response.
    ///
    /// # Returns
    /// - `StatusCodeUsize`: The HTTP status code as a usize (e.g., 200 for OK, 404 for Not Found).
    pub fn get_status_code(&self) -> StatusCodeUsize {
        self.status_code
    }

    /// Retrieves the status text associated with the HTTP status code.
    ///
    /// # Returns
    /// - `String`: The human-readable status text (e.g., "OK" for status code 200, "Not Found" for status code 404).
    pub fn get_status_text(&self) -> String {
        if let Ok(status_text) = self.status_text.read() {
            return status_text.to_string();
        }
        return StatusCode::default().to_string();
    }

    /// Retrieves the headers of the HTTP response.
    ///
    /// # Returns
    /// - `HttpHeaderMap`: A map of header names and their corresponding values as key-value pairs.
    pub fn get_headers(&self) -> HttpHeaderMap {
        if let Ok(headers) = self.headers.read() {
            return headers.clone();
        }
        return HttpHeaderMap::new();
    }

    /// Retrieves the body content of the HTTP response as a `String`.
    ///
    /// This method attempts to read the body of the response. If the body can be successfully read,
    /// it is converted into a `String` and returned. If reading the body fails, an empty string is returned.
    ///
    /// # Returns
    /// - `HttpBodyString`: The body of the response as a string. If the body could not be read,
    ///   an empty string is returned.
    pub fn get_body(&self) -> HttpBodyString {
        if let Ok(body) = self.body.read() {
            return body.to_string();
        }
        return HttpBodyString::new();
    }
}

impl Default for HttpResponseText {
    fn default() -> Self {
        Self {
            http_version: Arc::new(RwLock::new(HttpVersion::Unknown(String::new()))),
            status_code: StatusCode::Unknown.code(),
            status_text: Arc::new(RwLock::new(StatusCode::Unknown.to_string())),
            headers: Arc::new(RwLock::new(HashMap::new())),
            body: Arc::new(RwLock::new(String::new())),
        }
    }
}
