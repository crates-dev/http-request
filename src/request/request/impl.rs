use crate::*;

/// Implements methods for the `HttpRequest` struct.
///
/// These methods provide functionality for managing HTTP requests, including:
/// - Retrieving or setting HTTP attributes (e.g., URL, headers, protocol).
/// - Constructing and sending HTTP GET or POST requests.
/// - Parsing responses and handling redirects.
impl HttpRequest {
    /// Returns the protocol of the HTTP request.
    pub(crate) fn get_protocol(config: &Config) -> Protocol {
        config.url_obj.protocol.clone()
    }

    /// Returns the HTTP method used for the request.
    pub(crate) fn get_methods(&self) -> Method {
        self.methods.as_ref().clone()
    }

    /// Returns the URL of the HTTP request.
    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    /// Returns the headers of the HTTP request.
    fn get_header(&self) -> RequestHeaders {
        self.header.as_ref().clone()
    }

    /// Returns the body of the HTTP request.
    fn get_body(&self) -> Body {
        self.body.as_ref().clone()
    }

    /// Sets the URL for the HTTP request.
    ///
    /// # Parameters
    ///
    /// - `url`: The new URL to set.
    pub(crate) fn url(&mut self, url: String) {
        self.url = Arc::new(url);
    }

    /// Parses the current URL into a `HttpUrlComponents` object.
    ///
    /// Returns `Ok(HttpUrlComponents)` if the parsing succeeds, or `Err(RequestError::InvalidUrl)` otherwise.
    pub(crate) fn parse_url(&self) -> Result<HttpUrlComponents, RequestError> {
        match HttpUrlComponents::parse(&self.get_url()) {
            Ok(parse_res) => Ok(parse_res),
            Err(err) => Err(RequestError::InvalidUrl(err.to_string())),
        }
    }

    /// Converts the HTTP headers into a formatted HTTP header string and returns it as a byte vector.
    ///
    /// This method processes the HTTP headers by combining both user-defined and required headers.
    /// Required headers such as `Host`, `Content-Length`, `Accept`, and `User-Agent` are added if
    /// they are missing, with appropriate default values. The headers are then formatted into
    /// the standard HTTP header format and converted into a vector of bytes.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the formatted HTTP headers as a byte sequence.
    ///
    /// # Notes
    ///
    /// - The `Host` header is derived from the URL's host in the configuration.
    /// - The `Content-Length` header is calculated based on the request method:
    ///   - For `GET` requests, it is set to `0`.
    ///   - For other methods, it is determined by the length of the body.
    /// - If any required header is missing, it is automatically added with its default value.
    /// - Headers are concatenated into a string with each header ending in a line break specified by `HTTP_BR`.
    ///
    /// # Behavior
    ///
    /// This function ensures that all necessary headers are present and correctly formatted
    /// before constructing the HTTP request.
    pub(crate) fn get_header_bytes(&self) -> Vec<u8> {
        let mut header: RequestHeaders = self.get_header();

        let body_length: usize = if self.get_methods().is_get() {
            0usize
        } else {
            self.get_body_bytes().len()
        };

        if let Ok(config) = self.config.read() {
            let host_value: String = config.url_obj.host.clone().unwrap_or_default();
            let content_length_value: String = body_length.to_string();

            if !header.contains_key(HOST) {
                header.insert(HOST.to_owned(), host_value);
            }
            if !header.contains_key(CONTENT_LENGTH) {
                header.insert(CONTENT_LENGTH.to_owned(), content_length_value);
            }
            if !header.contains_key(ACCEPT) {
                header.insert(ACCEPT.to_owned(), ACCEPT_ANY.to_owned());
            }
            if !header.contains_key(USER_AGENT) {
                header.insert(USER_AGENT.to_owned(), APP_NAME.to_owned());
            }
        }

        let estimated_size: usize = header.iter().map(|(k, v)| k.len() + v.len() + 4).sum();
        let mut header_bytes: Vec<u8> = Vec::with_capacity(estimated_size);

        for (key, value) in &header {
            header_bytes.extend_from_slice(key.as_bytes());
            header_bytes.extend_from_slice(b": ");
            header_bytes.extend_from_slice(value.as_bytes());
            header_bytes.extend_from_slice(HTTP_BR_BYTES);
        }

        header_bytes
    }

    /// Converts the HTTP body into a URL-encoded byte vector (`Vec<u8>`).
    ///
    /// This method processes the body of the HTTP request based on the `Content-Type` header.
    /// If the `Content-Type` is valid and supports conversion, the body is transformed into a
    /// URL-encoded string and returned as a vector of bytes.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the URL-encoded representation of the HTTP body.
    /// If the `Content-Type` is not recognized or if the body cannot be converted,
    /// an empty byte vector is returned.
    ///
    /// # Notes
    ///
    /// The `Content-Type` header is matched case-insensitively. If no matching `Content-Type`
    /// is found or the parsing fails, the method defaults to returning an empty byte vector.
    /// The body processing relies on the implementation of the `ContentType` parsing logic.
    pub(crate) fn get_body_bytes(&self) -> Vec<u8> {
        let header: RequestHeaders = self.get_header();
        let body: Body = self.get_body();

        if let Some(content_type_value) = header.get(CONTENT_TYPE) {
            let res: String = content_type_value
                .to_lowercase()
                .parse::<ContentType>()
                .unwrap_or_default()
                .get_body_string(&body);
            return res.into_bytes();
        }

        for (key, value) in &header {
            if key.eq_ignore_ascii_case(CONTENT_TYPE) {
                let res: String = value
                    .to_lowercase()
                    .parse::<ContentType>()
                    .unwrap_or_default()
                    .get_body_string(&body);
                return res.into_bytes();
            }
        }

        String::new().into_bytes()
    }

    /// Retrieves the full path of the HTTP request, including the query string if present.
    ///
    /// This function constructs and returns the complete path of the HTTP request, which
    /// is composed of the path and, if available, the query string. The method checks if
    /// the `url_obj` contains a query string, and if it does, appends it to the path using
    /// the appropriate query separator (`?`). If no query string is present, only the
    /// path is returned.
    ///
    /// The function defaults to a predefined path (`DEFAULT_HTTP_PATH`) if the path is
    /// not set in the `url_obj` configuration. If the query string is empty, the function
    /// simply returns the path.
    ///
    /// # Returns
    ///
    /// - `String` - The full path, including the query string if available, or just the
    ///   path if no query string is present.
    pub(crate) fn get_path(&self) -> String {
        let path: String = self.config.read().map_or(String::new(), |config| {
            let query: String = config.url_obj.query.clone().unwrap_or_default();
            if query.is_empty() {
                config
                    .url_obj
                    .path
                    .clone()
                    .unwrap_or(DEFAULT_HTTP_PATH.to_string())
            } else {
                format!(
                    "{}{}{}",
                    config.url_obj.path.clone().unwrap_or_default(),
                    QUERY_SYMBOL,
                    query
                )
            }
        });
        path
    }

    /// Sends a GET request over the provided stream and returns the HTTP response.
    ///
    /// This method constructs and sends an HTTP GET request to the server. It formats the URL path
    /// and query parameters based on the current configuration and sends the request to the server
    /// via the provided `stream`. After sending the request, it waits for the response and reads
    /// the result.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to a `Box<dyn ReadWrite>`, representing the stream used
    ///   for sending and receiving data.
    ///
    /// # Returns
    /// Returns a `Result<HttpResponseBinary, RequestError>`, where:
    /// - `Ok(HttpResponseBinary)` contains the HTTP response received from the server.
    /// - `Err(RequestError)` indicates that an error occurred while sending the request or reading the response.
    fn send_get_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let path: String = self.get_path();
        let header_bytes: Vec<u8> = self.get_header_bytes();
        let method_str: &str = "GET";
        let http_version_str: String =
            self.config.read().map_or("HTTP/1.1".to_string(), |config| {
                config.http_version.to_string()
            });
        let request_line_size: usize =
            method_str.len() + 1 + path.len() + 1 + http_version_str.len();
        let total_size: usize = request_line_size + 2 + header_bytes.len() + 2;
        let mut request: Vec<u8> = Vec::with_capacity(total_size);
        request.extend_from_slice(method_str.as_bytes());
        request.push(b' ');
        request.extend_from_slice(path.as_bytes());
        request.push(b' ');
        request.extend_from_slice(http_version_str.as_bytes());
        request.extend_from_slice(HTTP_BR_BYTES);
        request.extend_from_slice(&header_bytes);
        request.extend_from_slice(HTTP_BR_BYTES);
        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|err| RequestError::Request(err.to_string()))?;
        self.read_response(stream)
    }

    /// Sends a POST request over the provided stream and returns the HTTP response.
    ///
    /// This method constructs and sends an HTTP POST request to the server. It formats the URL path
    /// and sends the body content along with the headers to the server via the provided `stream`. After
    /// sending the request, it waits for the response and reads the result.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to a `Box<dyn ReadWrite>`, representing the stream used
    ///   for sending and receiving data.
    ///
    /// # Returns
    /// Returns a `Result<HttpResponseBinary, RequestError>`, where:
    /// - `Ok(HttpResponseBinary)` contains the HTTP response received from the server.
    /// - `Err(RequestError)` indicates that an error occurred while sending the request or reading the response.
    fn send_post_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let path: String = self.get_path();
        let header_bytes: Vec<u8> = self.get_header_bytes();
        let body_bytes: Vec<u8> = self.get_body_bytes();

        let method_str: &str = "POST";
        let http_version_str: String =
            self.config.read().map_or("HTTP/1.1".to_string(), |config| {
                config.http_version.to_string()
            });

        let request_line_size: usize =
            method_str.len() + 1 + path.len() + 1 + http_version_str.len();
        let total_size: usize = request_line_size + 2 + header_bytes.len() + 2 + body_bytes.len();

        let mut request: Vec<u8> = Vec::with_capacity(total_size);

        request.extend_from_slice(method_str.as_bytes());
        request.push(b' ');
        request.extend_from_slice(path.as_bytes());
        request.push(b' ');
        request.extend_from_slice(http_version_str.as_bytes());
        request.extend_from_slice(HTTP_BR_BYTES);

        request.extend_from_slice(&header_bytes);
        request.extend_from_slice(HTTP_BR_BYTES);
        request.extend_from_slice(&body_bytes);

        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|err| RequestError::Request(err.to_string()))?;
        self.read_response(stream)
    }

    /// Reads the HTTP response from the provided stream.
    ///
    /// This method reads the response from the server after sending an HTTP request. It processes the
    /// headers, checks for redirects, and retrieves the response body based on the content length.
    /// If a redirect is detected, it follows the redirection URL. The method ensures that the entire
    /// response is read before returning.
    ///
    /// # Parameters
    /// - `stream`: A mutable reference to a `Box<dyn ReadWrite>`, representing the stream used
    ///   for receiving the response.
    ///
    /// # Returns
    /// Returns a `Result<HttpResponseBinary, RequestError>`, where:
    /// - `Ok(HttpResponseBinary)` contains the complete HTTP response after processing headers and body.
    /// - `Err(RequestError)` indicates that an error occurred while reading the response.
    fn read_response(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let buffer_size: usize = self
            .config
            .read()
            .map_or(DEFAULT_BUFFER_SIZE, |config| config.buffer);
        let mut buffer: Vec<u8> = vec![0; buffer_size];

        let initial_capacity: usize = buffer_size.max(8192);
        let mut response_bytes: Vec<u8> = Vec::with_capacity(initial_capacity);

        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<Vec<u8>> = None;
        let mut headers_end_pos: usize = 0;

        let http_version: String = self
            .config
            .read()
            .map_or(HttpVersion::default().to_string(), |config| {
                config.http_version.to_string()
            });
        let http_version_bytes: Vec<u8> = http_version.to_lowercase().into_bytes();
        let location_sign_key: Vec<u8> = format!("{}:", LOCATION.to_lowercase()).into_bytes();

        'read_loop: while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }

            if response_bytes.len() + n > response_bytes.capacity() {
                let new_capacity: usize =
                    (response_bytes.capacity() * 2).max(response_bytes.len() + n);
                response_bytes.reserve(new_capacity - response_bytes.capacity());
            }

            let old_len: usize = response_bytes.len();
            response_bytes.extend_from_slice(&buffer[..n]);

            if !headers_done {
                let search_start: usize = old_len.saturating_sub(3);
                if let Some(pos) = Self::find_double_crlf(&response_bytes, search_start) {
                    headers_done = true;
                    headers_end_pos = pos + 4;

                    self.parse_response_headers(
                        &response_bytes[..headers_end_pos],
                        &http_version_bytes,
                        &location_sign_key,
                        &mut content_length,
                        &mut redirect_url,
                    )?;
                }
            }

            if headers_done {
                let total_expected_length: usize = headers_end_pos + content_length;
                if response_bytes.len() >= total_expected_length {
                    response_bytes.truncate(total_expected_length);
                    break 'read_loop;
                }
            }
        }
        self.response = Arc::new(RwLock::new(<HttpResponseBinary as ResponseTrait>::from(
            &response_bytes,
        )));
        if let Ok(config) = self.config.read() {
            if !config.redirect || redirect_url.is_none() {
                if config.decode {
                    if let Ok(mut response) = self.response.write() {
                        *response = response.decode(config.buffer);
                    }
                }
                return Ok(Box::new(
                    self.response
                        .read()
                        .map_or(HttpResponseBinary::default(), |response| response.clone()),
                ));
            }
        }
        let url: String = String::from_utf8(redirect_url.unwrap())
            .map_err(|err| RequestError::InvalidUrl(err.to_string()))?;
        self.handle_redirect(url)
    }

    pub(crate) fn parse_response_headers(
        &self,
        headers_bytes: &[u8],
        http_version_bytes: &[u8],
        location_sign_key: &[u8],
        content_length: &mut usize,
        redirect_url: &mut Option<Vec<u8>>,
    ) -> Result<(), RequestError> {
        if let Some(status_pos) =
            Self::find_pattern_case_insensitive(headers_bytes, http_version_bytes)
        {
            let status_code_start: usize = status_pos + http_version_bytes.len() + 1;
            let status_code_end: usize = status_code_start + 3;

            if status_code_end <= headers_bytes.len() {
                let status_code: usize =
                    Self::parse_status_code(&headers_bytes[status_code_start..status_code_end]);

                if (300..=399).contains(&status_code) {
                    if let Some(location_pos) =
                        Self::find_pattern_case_insensitive(headers_bytes, location_sign_key)
                    {
                        let start: usize = location_pos + location_sign_key.len();
                        if let Some(end_pos) = Self::find_crlf(headers_bytes, start) {
                            let mut url_vec = Vec::with_capacity(end_pos - start);
                            url_vec.extend_from_slice(&headers_bytes[start..end_pos]);
                            *redirect_url = Some(url_vec);
                        }
                    }
                }
            }
        }

        *content_length = Self::get_content_length(headers_bytes);
        Ok(())
    }

    fn find_pattern_case_insensitive(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() || haystack.len() < needle.len() {
            return None;
        }

        let needle_len: usize = needle.len();
        let search_len: usize = haystack.len() - needle_len + 1;
        let first_needle_lower: u8 = needle[0].to_ascii_lowercase();

        'outer: for i in 0..search_len {
            if haystack[i].to_ascii_lowercase() != first_needle_lower {
                continue;
            }

            for j in 1..needle_len {
                if haystack[i + j].to_ascii_lowercase() != needle[j].to_ascii_lowercase() {
                    continue 'outer;
                }
            }
            return Some(i);
        }
        None
    }

    fn find_crlf(data: &[u8], start: usize) -> Option<usize> {
        let search_data: &[u8] = &data[start..];
        for i in 0..search_data.len().saturating_sub(1) {
            if search_data[i] == b'\r' && search_data[i + 1] == b'\n' {
                return Some(start + i);
            }
        }
        None
    }

    pub(crate) fn find_double_crlf(data: &[u8], start: usize) -> Option<usize> {
        let search_data: &[u8] = &data[start..];
        for i in 0..search_data.len().saturating_sub(3) {
            if search_data[i] == b'\r'
                && search_data[i + 1] == b'\n'
                && search_data[i + 2] == b'\r'
                && search_data[i + 3] == b'\n'
            {
                return Some(start + i);
            }
        }
        None
    }

    fn get_content_length(response_bytes: &[u8]) -> usize {
        const CONTENT_LENGTH_PATTERN: &[u8] = b"content-length:";

        if let Some(pos) =
            Self::find_pattern_case_insensitive(response_bytes, CONTENT_LENGTH_PATTERN)
        {
            let value_start: usize = pos + CONTENT_LENGTH_PATTERN.len();

            let value_start: usize = if response_bytes.get(value_start) == Some(&b' ') {
                value_start + 1
            } else {
                value_start
            };

            if let Some(end_pos) = Self::find_crlf(response_bytes, value_start) {
                let value_bytes: &[u8] = &response_bytes[value_start..end_pos];
                return Self::parse_decimal_bytes(value_bytes);
            }
        }
        0
    }

    /// Parses the status code from a byte slice.
    ///
    /// This function extracts and parses the HTTP status code from the response bytes.
    ///
    /// # Parameters
    /// - `status_bytes`: A byte slice containing the status code as a string.
    ///
    /// # Returns
    /// Returns the parsed status code as `usize`. If parsing fails, returns `0`.
    fn parse_decimal_bytes(bytes: &[u8]) -> usize {
        let mut result: usize = 0;
        let mut started: bool = false;

        for &byte in bytes {
            match byte {
                b'0'..=b'9' => {
                    started = true;
                    result = result * 10 + (byte - b'0') as usize;
                }
                b' ' | b'\t' if !started => continue,
                _ => break,
            }
        }
        result
    }

    fn parse_status_code(status_bytes: &[u8]) -> usize {
        if status_bytes.len() != 3 {
            return 0;
        }

        let mut result: usize = 0;
        for &byte in status_bytes {
            if byte >= b'0' && byte <= b'9' {
                result = result * 10 + (byte - b'0') as usize;
            } else {
                return 0;
            }
        }
        result
    }

    /// Handles HTTP redirects by following the redirection URL.
    ///
    /// # Parameters
    ///
    /// - `url`: The redirection URL to follow.
    ///
    /// Returns `Ok(HttpResponseBinary)` if the redirection is successful, or `Err(RequestError)` otherwise.
    fn handle_redirect(&mut self, url: String) -> Result<BoxResponseTrait, RequestError> {
        if let Ok(mut config) = self.config.write() {
            if !config.redirect {
                return Err(RequestError::NeedOpenRedirect);
            }
            if let Ok(mut tmp) = self.tmp.clone().write() {
                if tmp.visit_url.contains(&url) {
                    return Err(RequestError::RedirectUrlDeadLoop);
                }
                tmp.visit_url.insert(url.clone());
                if config.redirect_times >= config.max_redirect_times {
                    return Err(RequestError::MaxRedirectTimes);
                }
                config.redirect_times += 1;
            }
        }
        self.url(url.clone());
        self.send_sync()
    }

    /// Determines the appropriate port for the HTTP request.
    ///
    /// # Parameters
    ///
    /// - `port`: The default port (if any).
    /// - `config`: Config
    ///
    /// Returns the resolved port.
    pub(crate) fn get_port(&self, port: u16, config: &Config) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol: Protocol = Self::get_protocol(config);
        protocol.get_port()
    }

    /// Establishes a connection stream to the specified host and port.
    ///
    /// This method attempts to create a connection stream based on the protocol type
    /// (HTTP or HTTPS) defined by the current configuration. It supports both plain
    /// TCP connections and TLS-secured connections. If the protocol is HTTPS, it will
    /// use the `TlsConnector` to establish a secure TLS connection. For both cases,
    /// it ensures a read timeout is set on the stream.
    ///
    /// # Parameters
    ///
    /// - `host`: The hostname or IP address to connect to.
    /// - `port`: The port number to connect to.
    ///
    /// # Returns
    ///
    /// - `Ok(Box<dyn ReadWrite>)`: A boxed stream that implements the `ReadWrite` trait,
    ///   representing the established connection.
    /// - `Err(RequestError)`: An error indicating what went wrong during the connection process.
    fn get_connection_stream(
        &self,
        host: String,
        port: u16,
    ) -> Result<Box<dyn ReadWrite>, RequestError> {
        let host_port: (String, u16) = (host.clone(), port);
        let timeout: Duration = Duration::from_millis(
            self.config
                .read()
                .map_or(DEFAULT_TIMEOUT, |config| config.timeout),
        );
        let tcp_stream: TcpStream = TcpStream::connect(host_port.clone())
            .map_err(|err| RequestError::TcpStreamConnect(err.to_string()))?;
        tcp_stream
            .set_read_timeout(Some(timeout))
            .map_err(|err| RequestError::SetReadTimeout(err.to_string()))?;
        tcp_stream
            .set_write_timeout(Some(timeout))
            .map_err(|err| RequestError::SetWriteTimeout(err.to_string()))?;
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        let stream: Result<Box<dyn ReadWrite>, RequestError> =
            if Self::get_protocol(&config).is_https() {
                match self.tmp.clone().read() {
                    Ok(tmp) => {
                        let roots: RootCertStore = tmp.root_cert.clone();
                        let config: ClientConfig = ClientConfig::builder()
                            .with_root_certificates(roots)
                            .with_no_client_auth();
                        let client_config: Arc<ClientConfig> = Arc::new(config);
                        let dns_name: ServerName<'_> = ServerName::try_from(host.clone())
                            .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
                        let session: ClientConnection =
                            ClientConnection::new(Arc::clone(&client_config), dns_name)
                                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
                        let tls_stream: StreamOwned<ClientConnection, TcpStream> =
                            StreamOwned::new(session, tcp_stream);
                        return Ok(Box::new(tls_stream));
                    }
                    Err(err) => Err(RequestError::Unknown(format!(
                        "error reading temporary configuration: {}",
                        err
                    ))),
                }
            } else {
                Ok(Box::new(tcp_stream))
            };
        stream
    }
}

impl HttpRequest {
    /// Sends the HTTP request synchronously.
    pub fn send_sync(&mut self) -> RequestResult {
        let methods: Method = self.get_methods();
        let mut host: String = String::new();
        let mut port: u16 = u16::default();
        if let Ok(mut config) = self.config.write() {
            config.url_obj = self
                .parse_url()
                .map_err(|err| RequestError::InvalidUrl(err.to_string()))?;
            host = config.url_obj.host.clone().unwrap_or_default();
            port = self.get_port(config.url_obj.port.clone().unwrap_or_default(), &config);
        }
        let mut stream: BoxReadWrite = self.get_connection_stream(host, port)?;
        let res: Result<BoxResponseTrait, RequestError> = match methods {
            m if m.is_get() => self.send_get_request(&mut stream),
            m if m.is_post() => self.send_post_request(&mut stream),
            err => Err(RequestError::Request(format!(
                "do not support {} method",
                err
            ))),
        };
        res
    }
}

impl RequestTrait for HttpRequest {
    type RequestResult = RequestResult;
    fn send(&mut self) -> Self::RequestResult {
        self.send_sync()
    }
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            methods: Arc::new(Method::new()),
            url: Arc::new(String::new()),
            header: Arc::new(hash_map_xx_hash3_64()),
            body: Arc::new(Body::default()),
            config: Arc::new(RwLock::new(Config::default())),
            tmp: Arc::new(RwLock::new(Tmp::default())),
            response: Arc::new(RwLock::new(HttpResponseBinary::default())),
        }
    }
}
