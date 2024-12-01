use super::r#type::HttpResponseBinary;
use crate::{
    constant::{
        common::{BR_BYTES, COLON_SPACE_BYTES},
        http::{CONTENT_LENGTH, HTTP_BR, HTTP_BR_BYTES},
    },
    request::http_version::r#type::HttpVersion,
    response::http_response_text::r#type::HttpResponseText,
    status_code::r#type::StatusCode,
    utils::vec::{split_multi_byte, split_whitespace},
};
use std::{collections::HashMap, vec::IntoIter};

/// Provides functionality for parsing and working with HTTP responses.
///
/// This implementation contains methods for extracting specific information from HTTP response
/// strings, such as content length, and parsing the entire response into an `HttpResponseBinary` object.
///
/// # Methods
/// - `get_content_length`: Extracts the `Content-Length` value from the HTTP response string.
/// - `from`: Parses a raw HTTP response string into an `HttpResponseBinary` struct, including the
///   status line, headers, and body.
impl HttpResponseBinary {
    /// Extracts the `Content-Length` from the response string.
    ///
    /// This method scans the HTTP response string for the `Content-Length` header and parses
    /// its value into a `usize`. If the header is not present or its value is invalid, the method
    /// returns `0` as a default.
    ///
    /// # Parameters
    /// - `response_string`: A string representing the HTTP response.
    ///
    /// # Returns
    /// Returns the `Content-Length` value extracted from the response, or `0` if not found.
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

    /// Parses an HTTP response from a byte slice and returns an `HttpResponseBinary` object.
    ///
    /// This function processes the raw HTTP response in byte form. It splits the response into
    /// the status line, headers, and body, parsing each part accordingly. The status line is parsed
    /// to extract the HTTP version, status code, and status text. Headers are split and stored in
    /// a `HashMap`. The body is collected into a byte vector.
    ///
    /// # Parameters
    /// - `response`: A byte slice representing the raw HTTP response.
    ///
    /// # Returns
    /// Returns an `HttpResponseBinary` object containing the parsed HTTP version, status code, status text,
    /// headers, and body. If parsing any part fails, defaults are used (e.g., `HTTP/1.1`, status code `200`).
    ///
    /// # Panics
    /// This method will panic if the HTTP response is malformed in ways that the unwrap operations cannot handle.
    pub fn from(response: &[u8]) -> Self {
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
            http_version,
            status_code,
            status_text,
            headers,
            body,
        }
    }

    /// Converts the response body to text format.
    ///
    /// This function takes the current response and creates a new `HttpResponseBinary`
    /// instance with the body converted to a text representation. The `body` is
    /// extracted as text from the original response body and stored in the new
    /// response as a `ResponseBody::Text` variant.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `HttpResponseBinary` instance with the body converted to text.
    pub fn text(self) -> HttpResponseText {
        let body: String = String::from_utf8_lossy(&self.body).to_string();
        HttpResponseText {
            http_version: self.http_version,
            status_code: self.status_code,
            status_text: self.status_text,
            headers: self.headers,
            body,
        }
    }
}

/// Default implementation for `HttpResponseBinary`.
///
/// This implementation provides default values for an `HttpResponseBinary` instance, setting the HTTP
/// version to the default version, the status code to `StatusCode::Unknown`, and initializing the
/// headers and body to empty collections.
impl Default for HttpResponseBinary {
    fn default() -> Self {
        HttpResponseBinary {
            http_version: HttpVersion::Unknown(String::new()).to_string(),
            status_code: StatusCode::Unknown.code(),
            status_text: StatusCode::Unknown.to_string(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}
