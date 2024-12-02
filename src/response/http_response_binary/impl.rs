use super::r#type::HttpResponseBinary;
use crate::{
    constant::{
        common::{BR_BYTES, COLON_SPACE_BYTES},
        http::HTTP_BR_BYTES,
    },
    request::http_version::r#type::HttpVersion,
    response::{http_response_text::r#type::HttpResponseText, r#trait::HttpResponse},
    status_code::r#type::StatusCode,
    utils::vec::{split_multi_byte, split_whitespace},
};
use std::{collections::HashMap, vec::IntoIter};

/// Implements the `HttpResponse` trait for `HttpResponseBinary`.
///
/// This implementation specifies the associated types for binary and text representations
/// of HTTP responses, enabling seamless conversion and handling of HTTP response data.
///
/// # Associated Types
/// - `OutputText`: Specifies the text representation of an HTTP response (`HttpResponseText`).
/// - `OutputBinary`: Specifies the binary representation of an HTTP response (`HttpResponseBinary`).
impl HttpResponse for HttpResponseBinary {
    type OutputText = HttpResponseText;
    type OutputBinary = HttpResponseBinary;

    /// Parses a raw HTTP response from a byte slice and constructs an `HttpResponseBinary` instance.
    ///
    /// This method processes the raw HTTP response into its constituent parts: status line, headers, and body.
    /// Each part is parsed and stored in the resulting `HttpResponseBinary` object.
    ///
    /// # Parameters
    /// - `response`: A byte slice representing the raw HTTP response.
    ///
    /// # Returns
    /// - `HttpResponseBinary`: A structured representation of the parsed HTTP response, including
    ///   the HTTP version, status code, status text, headers, and body.
    ///
    /// # Panics
    /// - This method will panic if the HTTP response format is malformed and required components
    ///   such as the status line cannot be parsed.
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
            http_version,
            status_code,
            status_text,
            headers,
            body,
        }
    }

    /// Returns a clone of the binary representation of the HTTP response.
    ///
    /// This method is part of the `HttpResponse` trait implementation, allowing for retrieval
    /// of the current binary HTTP response without modification.
    ///
    /// # Returns
    /// - `Self::OutputBinary`: A binary representation of the HTTP response, cloned from the current instance.
    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    /// Converts the binary HTTP response to its text representation.
    ///
    /// This method processes the current instance of `HttpResponseBinary` by interpreting the
    /// response body as UTF-8 encoded text, preserving other components such as HTTP version,
    /// status code, status text, and headers unchanged.
    ///
    /// # Returns
    /// - `HttpResponseText`: A structured representation of the HTTP response with the body
    ///   converted to text.
    fn text(&self) -> HttpResponseText {
        let http_response: HttpResponseBinary = self.clone();
        let body: String = String::from_utf8_lossy(&http_response.body).to_string();
        HttpResponseText {
            http_version: http_response.http_version,
            status_code: http_response.status_code,
            status_text: http_response.status_text,
            headers: http_response.headers,
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
