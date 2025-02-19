use crate::*;

/// Implements methods for the `HttpRequest` struct.
///
/// These methods provide functionality for managing HTTP requests, including:
/// - Retrieving or setting HTTP attributes (e.g., URL, headers, protocol).
/// - Constructing and sending HTTP GET or POST requests.
/// - Parsing responses and handling redirects.
impl HttpRequest {
    /// Returns the protocol of the HTTP request.
    #[inline]
    fn get_protocol(config: &Config) -> Protocol {
        config.url_obj.protocol.clone()
    }

    /// Returns the HTTP method used for the request.
    #[inline]
    fn get_methods(&self) -> Methods {
        self.methods.as_ref().clone()
    }

    /// Returns the URL of the HTTP request.
    #[inline]
    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    /// Returns the headers of the HTTP request.
    #[inline]
    fn get_header(&self) -> HttpHeaderSliceMap {
        self.header.as_ref().clone()
    }

    /// Returns the body of the HTTP request.
    #[inline]
    fn get_body(&self) -> Body {
        self.body.as_ref().clone()
    }

    /// Sets the URL for the HTTP request.
    ///
    /// # Parameters
    ///
    /// - `url`: The new URL to set.
    #[inline]
    fn url(&mut self, url: String) {
        self.url = Arc::new(url);
    }

    /// Parses the current URL into a `HttpUrlComponents` object.
    ///
    /// Returns `Ok(HttpUrlComponents)` if the parsing succeeds, or `Err(RequestError::InvalidUrl)` otherwise.
    #[inline]
    fn parse_url(&self) -> Result<HttpUrlComponents, RequestError> {
        if let Ok(parse_res) = HttpUrlComponents::parse(&self.get_url()) {
            Ok(parse_res)
        } else {
            Err(RequestError::InvalidUrl)
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
    #[inline]
    fn get_header_bytes(&self) -> Vec<u8> {
        let mut header: HttpHeaderSliceMap = self.get_header();
        let mut header_string: String = String::new();
        if let Ok(config) = self.config.read() {
            let required_headers = [
                (HOST, config.url_obj.host.clone().unwrap_or_default()),
                (
                    CONTENT_LENGTH,
                    if self.get_methods().is_get() {
                        ZERO_STR.to_string()
                    } else {
                        self.get_body_bytes().len().to_string()
                    },
                ),
                (ACCEPT, ACCEPT_ANY.to_owned()),
                (USER_AGENT, APP_NAME.to_owned()),
            ];
            for (key, default_value) in required_headers {
                if !header.contains_key(key) {
                    header.insert(key.to_owned(), default_value.to_owned());
                }
            }
        }
        for (key, value) in &header {
            header_string.push_str(&format!("{}: {}{}", key, value, HTTP_BR));
        }
        header_string.as_bytes().to_vec()
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
    #[inline]
    fn get_body_bytes(&self) -> Vec<u8> {
        let header: HttpHeaderSliceMap = self.get_header();
        let body: Body = self.get_body();
        let mut res: String = String::new();
        for (key, value) in header {
            if key.eq_ignore_ascii_case(CONTENT_TYPE) {
                res = value
                    .to_lowercase()
                    .parse::<ContentType>()
                    .unwrap_or_default()
                    .get_body_string(&body);
                break;
            }
        }
        res.into_bytes()
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
    #[inline]
    fn get_path(&self) -> String {
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
    #[inline]
    fn send_get_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let mut request: Vec<u8> = Vec::new();
        let path: String = self.get_path();
        let request_line_string: String = self.config.read().map_or(String::new(), |config| {
            format!("{} {} {}", Methods::GET, path, config.http_version)
        });
        let request_line: &[u8] = request_line_string.as_bytes();
        request.extend_from_slice(request_line);
        request.extend_from_slice(HTTP_BR_BYTES);
        request.extend_from_slice(&self.get_header_bytes());
        request.extend_from_slice(HTTP_BR_BYTES);
        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|_| RequestError::RequestError)?;
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
    #[inline]
    fn send_post_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let mut request: Vec<u8> = Vec::new();
        let path: String = self.get_path();
        let request_line_string: String = self.config.read().map_or(String::new(), |config| {
            format!("{} {} {}", Methods::POST, path, config.http_version)
        });
        let request_line: &[u8] = request_line_string.as_bytes();
        request.extend_from_slice(request_line);
        request.extend_from_slice(HTTP_BR_BYTES);
        request.extend_from_slice(&self.get_header_bytes());
        request.extend_from_slice(HTTP_BR_BYTES);
        let body_str: &Vec<u8> = &self.get_body_bytes();
        request.extend_from_slice(body_str);
        stream
            .write_all(&request)
            .and_then(|_| stream.flush())
            .map_err(|_| RequestError::RequestError)?;
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
    #[inline]
    fn read_response(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<BoxResponseTrait, RequestError> {
        let buffer_size: usize = self
            .config
            .read()
            .map_or(DEFAULT_BUFFER_SIZE, |config| config.buffer);
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        let mut response_bytes: Vec<u8> = Vec::with_capacity(buffer_size);
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<Vec<u8>> = None;
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
            response_bytes.extend_from_slice(&buffer[..n]);
            if !headers_done
                && response_bytes
                    .windows(HTTP_DOUBLE_BR_BYTES.len())
                    .any(|window| window == HTTP_DOUBLE_BR_BYTES)
            {
                headers_done = true;
                if let Some(status_pos) = response_bytes
                    .windows(http_version_bytes.len())
                    .position(|window| case_insensitive_match(window, &http_version_bytes))
                {
                    let status_code_start: usize = status_pos + http_version_bytes.len() + 1; // Skip "HTTP/1.1 "
                    let status_code_end: usize = status_code_start + 3; // Status code is 3 digits
                    let status_code: usize = Self::parse_status_code(
                        &response_bytes[status_code_start..status_code_end],
                    );
                    if (300..=399).contains(&status_code) {
                        if let Some(location_pos) = response_bytes
                            .windows(location_sign_key.len())
                            .position(|window| case_insensitive_match(window, &location_sign_key))
                        {
                            let start: usize = location_pos + location_sign_key.len();
                            if let Some(end_pos) = response_bytes[start..]
                                .windows(HTTP_BR_BYTES.len())
                                .position(|window| window == HTTP_BR_BYTES)
                            {
                                redirect_url =
                                    Some(response_bytes[start..start + end_pos].to_vec());
                            }
                        }
                    }
                    content_length = Self::get_content_length(&response_bytes);
                }
            }
            if headers_done && response_bytes.len() >= content_length {
                break 'read_loop;
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
        let url: String =
            String::from_utf8(redirect_url.unwrap()).map_err(|_| RequestError::InvalidUrl)?;
        self.handle_redirect(url)
    }

    /// Extracts the content length from the HTTP response bytes.
    ///
    /// This function searches for the `Content-Length` field in a case-insensitive
    /// manner and parses its value if found.
    ///
    /// # Parameters
    /// - `response_bytes`: A byte slice containing the raw HTTP response.
    ///
    /// # Returns
    /// Returns the parsed content length as `usize`. If not found or invalid, returns `0`.
    #[inline]
    fn get_content_length(response_bytes: &[u8]) -> usize {
        let content_length_key: Vec<u8> =
            format!("{}:", CONTENT_LENGTH.to_lowercase()).into_bytes();
        if let Some(pos) = Self::find_case_insensitive_key(response_bytes, &content_length_key) {
            if let Some(length_str) =
                Self::extract_value_from_position(response_bytes, pos, &HTTP_BR_BYTES)
            {
                return length_str.trim().parse::<usize>().unwrap_or(0);
            }
        }
        0
    }

    /// Finds the position of a key in the response bytes, case-insensitively.
    ///
    /// This function scans the `response_bytes` for the given `key` and returns
    /// the starting position of the match, if found.
    ///
    /// # Parameters
    /// - `response_bytes`: A byte slice containing the raw HTTP response.
    /// - `key`: The byte sequence to search for.
    ///
    /// # Returns
    /// Returns an `Option<usize>` containing the starting position if the key is found, otherwise `None`.
    #[inline]
    fn find_case_insensitive_key(response_bytes: &[u8], key: &[u8]) -> Option<usize> {
        response_bytes
            .windows(key.len())
            .position(|window| case_insensitive_match(window, key))
    }

    /// Extracts the value following a key at a specific position until the end delimiter.
    ///
    /// This function assumes the key ends with a colon (`:`) and extracts the value
    /// up to the specified delimiter.
    ///
    /// # Parameters
    /// - `response_bytes`: A byte slice containing the raw HTTP response.
    /// - `key_pos`: The starting position of the key in the response bytes.
    /// - `delimiter`: The byte sequence representing the end of the value.
    ///
    /// # Returns
    /// Returns an `Option<&str>` containing the extracted value. If not found or invalid, returns `None`.
    #[inline]
    fn extract_value_from_position<'a>(
        response_bytes: &'a [u8],
        key_pos: usize,
        delimiter: &'a [u8],
    ) -> Option<&'a str> {
        let start: usize = key_pos + delimiter.len();
        response_bytes[start..]
            .windows(delimiter.len())
            .position(|window| window == delimiter)
            .map(|end_pos| {
                std::str::from_utf8(&response_bytes[start..start + end_pos]).unwrap_or_default()
            })
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
    #[inline]
    fn parse_status_code(status_bytes: &[u8]) -> usize {
        let status_str: &str = std::str::from_utf8(status_bytes).unwrap_or_default();
        status_str.trim().parse::<usize>().unwrap_or_default()
    }

    /// Handles HTTP redirects by following the redirection URL.
    ///
    /// # Parameters
    ///
    /// - `url`: The redirection URL to follow.
    ///
    /// Returns `Ok(HttpResponseBinary)` if the redirection is successful, or `Err(RequestError)` otherwise.
    #[inline]
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
                config.redirect_times = config.redirect_times + 1;
            }
        }
        self.url(url.clone());
        self.send()
    }

    /// Determines the appropriate port for the HTTP request.
    ///
    /// # Parameters
    ///
    /// - `port`: The default port (if any).
    /// - `config`: Config
    ///
    /// Returns the resolved port.
    #[inline]
    fn get_port(&self, port: u16, config: &Config) -> u16 {
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
    ///
    /// # Errors
    ///
    /// - `RequestError::TlsConnectorBuildError`: If the TLS connector could not be built.
    /// - `RequestError::TcpStreamConnectError`: If the TCP connection could not be established.
    /// - `RequestError::SetReadTimeoutError`: If setting the read timeout failed.
    /// - `RequestError::TlsStreamConnectError`: If the TLS stream could not be established.
    #[inline]
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
            .map_err(|_| RequestError::TcpStreamConnectError)?;
        tcp_stream
            .set_read_timeout(Some(timeout))
            .map_err(|_| RequestError::SetReadTimeoutError)?;
        tcp_stream
            .set_write_timeout(Some(timeout))
            .map_err(|_| RequestError::SetWriteTimeoutError)?;
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        let stream: Result<Box<dyn ReadWrite>, RequestError> =
            if Self::get_protocol(&config).is_https() {
                if let Ok(tmp) = self.tmp.clone().read() {
                    let roots: RootCertStore = tmp.root_cert.clone();
                    let config: ClientConfig = ClientConfig::builder()
                        .with_root_certificates(roots)
                        .with_no_client_auth();
                    let client_config: Arc<ClientConfig> = Arc::new(config);
                    let dns_name: ServerName<'_> = ServerName::try_from(host.clone())
                        .map_err(|_| RequestError::TlsConnectorBuildError)?;
                    let session: ClientConnection =
                        ClientConnection::new(Arc::clone(&client_config), dns_name)
                            .map_err(|_| RequestError::TlsConnectorBuildError)?;
                    let tls_stream: StreamOwned<ClientConnection, TcpStream> =
                        StreamOwned::new(session, tcp_stream);
                    return Ok(Box::new(tls_stream));
                }
                Err(RequestError::TlsConnectorBuildError)
            } else {
                Ok(Box::new(tcp_stream))
            };
        stream
    }
}

impl RequestTrait for HttpRequest {
    type RequestResult = RequestResult;
    #[inline]
    fn send(&mut self) -> Self::RequestResult {
        let methods: Methods = self.get_methods();
        let mut host: String = String::new();
        let mut port: u16 = u16::default();
        if let Ok(mut config) = self.config.write() {
            config.url_obj = self.parse_url().map_err(|_| RequestError::InvalidUrl)?;
            host = config.url_obj.host.clone().unwrap_or_default();
            port = self.get_port(config.url_obj.port.clone().unwrap_or_default(), &config);
        }
        let mut stream: Box<dyn ReadWrite> = self
            .get_connection_stream(host, port)
            .map_err(|_| RequestError::TcpStreamConnectError)?;
        let res: Result<BoxResponseTrait, RequestError> = match methods {
            m if m.is_get() => self.send_get_request(&mut stream),
            m if m.is_post() => self.send_post_request(&mut stream),
            _ => Err(RequestError::RequestError),
        };
        return res;
    }
}

impl Default for HttpRequest {
    #[inline]
    fn default() -> Self {
        Self {
            methods: Arc::new(Methods::new()),
            url: Arc::new(String::new()),
            header: Arc::new(HashMap::new()),
            body: Arc::new(Body::default()),
            config: Arc::new(RwLock::new(Config::default())),
            tmp: Arc::new(RwLock::new(Tmp::default())),
            response: Arc::new(RwLock::new(HttpResponseBinary::default())),
        }
    }
}
