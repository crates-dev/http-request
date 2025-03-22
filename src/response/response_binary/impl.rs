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

    #[inline]
    fn from(response: &[u8]) -> Self
    where
        Self: Sized,
    {
        let split_lines: Vec<&[u8]> = split_multi_byte(response, HTTP_BR_BYTES);
        let mut lines: IntoIter<&[u8]> = split_lines.into_iter();
        let status_line: &[u8] = lines.next().unwrap_or(&[]);
        let status_parts: Vec<&[u8]> = split_whitespace(&status_line);
        let http_version: HttpVersion = String::from_utf8_lossy(
            status_parts
                .get(0)
                .unwrap_or(&HttpVersion::Unknown(String::new()).to_string().as_bytes()),
        )
        .to_string()
        .parse::<HttpVersion>()
        .unwrap_or_default();
        let status_code: ResponseStatusCode = status_parts
            .get(1)
            .and_then(|part| std::str::from_utf8(part).ok())
            .unwrap_or(&HttpStatus::Ok.to_string())
            .parse()
            .unwrap_or(HttpStatus::Unknown.code());
        let status_text: String = status_parts.get(2..).map_or_else(
            || HttpStatus::Unknown.to_string(),
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

    #[inline]
    fn binary(&self) -> Self::OutputBinary {
        self.clone()
    }

    #[inline]
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

    #[inline]
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
        )
        .into_owned();
        HttpResponseBinary {
            http_version: http_response.http_version,
            status_code: http_response.status_code,
            status_text: http_response.status_text,
            headers: http_response.headers,
            body: Arc::new(RwLock::new(body)),
        }
    }
}

impl HttpResponseBinary {
    /// Retrieves the HTTP version associated with this response.
    ///
    /// # Returns
    /// - `HttpVersion`: The HTTP version (e.g., HTTP/1.1, HTTP/2, etc.) used for the response.
    #[inline]
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
    #[inline]
    pub fn get_status_code(&self) -> ResponseStatusCode {
        self.status_code
    }

    /// Retrieves the status text associated with the HTTP status code.
    ///
    /// # Returns
    /// - `String`: The human-readable status text (e.g., "OK" for status code 200, "Not Found" for status code 404).
    #[inline]
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
    #[inline]
    pub fn get_headers(&self) -> ResponseHeaders {
        if let Ok(headers) = self.headers.read() {
            return headers.clone();
        }
        return ResponseHeaders::new();
    }

    /// Retrieves the body content of the HTTP response.
    ///
    /// # Returns
    /// - `RequestBody`: The body of the response in binary form (could be raw bytes, a stream, etc.).
    #[inline]
    pub fn get_body(&self) -> RequestBody {
        if let Ok(body) = self.body.read() {
            return body.clone();
        }
        return RequestBody::new();
    }
}

impl Default for HttpResponseBinary {
    #[inline]
    fn default() -> Self {
        Self {
            http_version: Arc::new(RwLock::new(HttpVersion::Unknown(String::new()))),
            status_code: HttpStatus::Unknown.code(),
            status_text: Arc::new(RwLock::new(HttpStatus::Unknown.to_string())),
            headers: Arc::new(RwLock::new(HashMap::new())),
            body: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
