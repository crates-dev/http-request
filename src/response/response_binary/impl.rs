use crate::*;

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

        let http_version: HttpVersion = status_parts
            .get(0)
            .and_then(|part: &&[u8]| std::str::from_utf8(part).ok())
            .and_then(|version_str: &str| version_str.parse::<HttpVersion>().ok())
            .unwrap_or_default();

        let status_code: ResponseStatusCode = status_parts
            .get(1)
            .and_then(|part: &&[u8]| std::str::from_utf8(part).ok())
            .and_then(|code_str: &str| code_str.parse().ok())
            .unwrap_or(HttpStatus::Unknown.code());

        let status_text: String = status_parts.get(2..).map_or_else(
            || HttpStatus::Unknown.to_string(),
            |parts: &[&[u8]]| {
                if parts.is_empty() {
                    HttpStatus::Unknown.to_string()
                } else if parts.len() == 1 {
                    String::from_utf8_lossy(parts[0]).into_owned()
                } else {
                    let total_len: usize =
                        parts.iter().map(|p: &&[u8]| p.len()).sum::<usize>() + parts.len() - 1;
                    let mut result: String = String::with_capacity(total_len);
                    for (i, part) in parts.iter().enumerate() {
                        if i > 0 {
                            result.push(' ');
                        }
                        result.push_str(&String::from_utf8_lossy(part));
                    }
                    result
                }
            },
        );

        let mut headers: HashMapXxHash3_64<String, String> = hash_map_xx_hash3_64();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }

            let mut colon_pos: Option<usize> = None;
            for (i, &byte) in line.iter().enumerate() {
                if byte == b':' {
                    colon_pos = Some(i);
                    break;
                }
            }

            if let Some(pos) = colon_pos {
                if pos > 0 && pos + 1 < line.len() {
                    let key_bytes: &[u8] = &line[..pos];
                    let value_start: usize = if line.get(pos + 1) == Some(&b' ') {
                        pos + 2
                    } else {
                        pos + 1
                    };
                    let value_bytes: &[u8] = &line[value_start..];

                    if let (Ok(key_str), Ok(value_str)) = (
                        std::str::from_utf8(key_bytes),
                        std::str::from_utf8(value_bytes),
                    ) {
                        headers.insert(key_str.trim().to_string(), value_str.trim().to_string());
                    }
                }
            }
        }

        let body: Vec<u8> = match lines.len() {
            0 => Vec::new(),
            1 => {
                let line = lines.next().unwrap_or(&[]);
                let mut body = Vec::with_capacity(line.len());
                body.extend_from_slice(line);
                body
            }
            _ => {
                let lines_slice = lines.as_slice();
                let total_size: usize = lines_slice
                    .iter()
                    .map(|line: &&[u8]| line.len())
                    .sum::<usize>()
                    + lines_slice.len().saturating_sub(1) * BR_BYTES.len();

                let mut body: Vec<u8> = Vec::with_capacity(total_size);
                let mut first: bool = true;

                for line in lines {
                    if !first {
                        body.extend_from_slice(BR_BYTES);
                    }
                    body.extend_from_slice(line);
                    first = false;
                }
                body
            }
        };

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
        let body: String = self.body.read().map_or(String::new(), |body_ref| {
            String::from_utf8_lossy(&body_ref).into_owned()
        });

        HttpResponseText {
            http_version: Arc::clone(&self.http_version),
            status_code: self.status_code,
            status_text: Arc::clone(&self.status_text),
            headers: Arc::clone(&self.headers),
            body: Arc::new(RwLock::new(body)),
        }
    }

    fn decode(&self, buffer_size: usize) -> HttpResponseBinary {
        let decoded_body: Vec<u8> = {
            let headers_guard = self.headers.read();
            let body_guard = self.body.read();

            match (headers_guard, body_guard) {
                (Ok(headers_ref), Ok(body_ref)) => Compress::from(&*headers_ref)
                    .decode(&*body_ref, buffer_size)
                    .into_owned(),
                _ => Vec::new(),
            }
        };

        HttpResponseBinary {
            http_version: Arc::clone(&self.http_version),
            status_code: self.status_code,
            status_text: Arc::clone(&self.status_text),
            headers: Arc::clone(&self.headers),
            body: Arc::new(RwLock::new(decoded_body)),
        }
    }
}

impl HttpResponseBinary {
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
    /// - `ResponseStatusCode`: The HTTP status code as a usize (e.g., 200 for OK, 404 for Not Found).
    pub fn get_status_code(&self) -> ResponseStatusCode {
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
        return HttpStatus::default().to_string();
    }

    /// Retrieves the headers of the HTTP response.
    ///
    /// # Returns
    /// - `ResponseHeaders`: A map of header names and their corresponding values as key-value pairs.
    pub fn get_headers(&self) -> ResponseHeaders {
        if let Ok(headers) = self.headers.read() {
            return headers.clone();
        }
        return hash_map_xx_hash3_64();
    }

    /// Retrieves the body content of the HTTP response.
    ///
    /// # Returns
    /// - `RequestBody`: The body of the response in binary form (could be raw bytes, a stream, etc.).
    pub fn get_body(&self) -> RequestBody {
        if let Ok(body) = self.body.read() {
            return body.clone();
        }
        return RequestBody::new();
    }
}

impl Default for HttpResponseBinary {
    fn default() -> Self {
        Self {
            http_version: Arc::new(RwLock::new(HttpVersion::Unknown(String::new()))),
            status_code: HttpStatus::Unknown.code(),
            status_text: Arc::new(RwLock::new(HttpStatus::Unknown.to_string())),
            headers: Arc::new(RwLock::new(hash_map_xx_hash3_64())),
            body: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
