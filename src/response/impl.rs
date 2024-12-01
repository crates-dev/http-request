use super::r#type::HttpResponse;
use crate::{
    constant::http::{CONTENT_LENGTH, HTTP_BR, HTTP_VERSION_1_1},
    status_code::r#type::StatusCode,
};
use std::collections::HashMap;
use std::str::Lines;

/// Provides functionality for parsing and working with HTTP responses.
///
/// This implementation contains methods for extracting specific information from HTTP response
/// strings, such as content length, and parsing the entire response into an `HttpResponse` object.
///
/// # Methods
/// - `get_content_length`: Extracts the `Content-Length` value from the HTTP response string.
/// - `from`: Parses a raw HTTP response string into an `HttpResponse` struct, including the
///   status line, headers, and body.
impl HttpResponse {
    /// Extracts the `Content-Length` from the response string.
    ///
    /// This method scans the HTTP response string for the `Content-Length` header and parses
    /// its value into a `usize`. If the header is not present or its value is invalid, the method
    /// returns `0` as a default.
    ///
    /// # Arguments
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

    /// Parses a raw HTTP response string into an `HttpResponse` object.
    ///
    /// This method takes a raw HTTP response string and splits it into different components,
    /// such as the HTTP version, status code, status text, headers, and body.
    ///
    /// # Arguments
    /// - `response`: A string representing the raw HTTP response.
    ///
    /// # Returns
    /// Returns an `HttpResponse` struct containing the parsed HTTP version, status code, status
    /// text, headers, and body.
    pub fn from(response: &str) -> Self {
        let mut lines: Lines<'_> = response.lines();
        let status_line: &str = lines.next().unwrap_or("");
        let status_parts: Vec<&str> = status_line.split_whitespace().collect();
        let http_version: String = status_parts.get(0).unwrap_or(&HTTP_VERSION_1_1).to_string();
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

/// Default implementation for `HttpResponse`.
///
/// This implementation provides default values for an `HttpResponse` instance, setting the HTTP
/// version to the default version, the status code to `StatusCode::Unknown`, and initializing the
/// headers and body to empty collections.
impl Default for HttpResponse {
    fn default() -> Self {
        HttpResponse {
            http_version: HTTP_VERSION_1_1.to_string(),
            status_code: StatusCode::Unknown.code(),
            status_text: StatusCode::Unknown.to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}
