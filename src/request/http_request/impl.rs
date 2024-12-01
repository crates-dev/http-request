use super::r#type::HttpRequest;
use crate::{
    body::r#type::Body,
    constant::{
        common::APP_NAME,
        http::{
            ACCEPT, ACCEPT_VALUE, CONTENT_TYPE, HTTP_BR, HTTP_BR_BYTES, HTTP_DOUBLE_BR_BYTES,
            QUERY_SYMBOL, USER_AGENT,
        },
    },
    content_type::r#type::ContentType,
    methods::r#type::Methods,
    protocol::r#type::Protocol,
    request::{
        config::r#type::Config, error::Error, request_url::r#type::RequestUrl, tmp::r#type::Tmp,
    },
    utils::vec::case_insensitive_match,
};
use crate::{
    constant::http::{CONTENT_LENGTH, DEFAULT_HTTP_PATH, HOST, LOCATION},
    global_trait::r#trait::ReadWrite,
    header::r#type::Header,
    response::r#type::HttpResponse,
};
use native_tls::{TlsConnector, TlsStream};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
    time::Duration,
};

/// Implements methods for the `HttpRequest` struct.
///
/// These methods provide functionality for managing HTTP requests, including:
/// - Retrieving or setting HTTP attributes (e.g., URL, headers, protocol).
/// - Constructing and sending HTTP GET or POST requests.
/// - Parsing responses and handling redirects.
impl HttpRequest {
    /// Returns the protocol of the HTTP request.
    fn get_protocol(&self) -> Protocol {
        self.config.url_obj.protocol.clone()
    }

    /// Returns the HTTP method used for the request.
    fn get_methods(&self) -> Methods {
        self.methods.as_ref().clone()
    }

    /// Returns the URL of the HTTP request.
    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    /// Returns the headers of the HTTP request.
    fn get_header(&self) -> Header {
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
    fn url(&mut self, url: String) {
        self.url = Arc::new(url);
    }

    /// Parses the current URL into a `RequestUrl` object.
    ///
    /// Returns `Ok(RequestUrl)` if the parsing succeeds, or `Err(Error::InvalidUrl)` otherwise.
    fn parse_url(&self) -> Result<RequestUrl, Error> {
        if let Ok(parse_res) = RequestUrl::parse(&self.get_url()) {
            Ok(parse_res)
        } else {
            Err(Error::InvalidUrl)
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
    fn get_header_bytes(&self) -> Vec<u8> {
        let mut header: HashMap<&str, &str> = self.get_header();
        let mut header_string = String::new();
        let required_headers: [(&str, &str); 4] = [
            (
                HOST,
                self.config.url_obj.host.as_deref().unwrap_or_default(),
            ),
            (
                CONTENT_LENGTH,
                if self.get_methods().is_get() {
                    &0.to_string()
                } else {
                    &self.get_body_bytes().len().to_string()
                },
            ),
            (ACCEPT, ACCEPT_VALUE),
            (USER_AGENT, APP_NAME),
        ];
        for &(key, default_value) in &required_headers {
            if !header.contains_key(key) {
                header.insert(key, default_value);
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
    fn get_body_bytes(&self) -> Vec<u8> {
        let content_type_key: String = CONTENT_TYPE.to_lowercase();
        let header: HashMap<&str, &str> = self.get_header();
        let body: Body = self.get_body();
        let mut res: String = String::new();
        for (key, value) in header {
            if key.to_lowercase() == content_type_key {
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
    fn get_path(&self) -> String {
        let query: String = self.config.url_obj.query.clone().unwrap_or_default();
        let path: String = if query.is_empty() {
            self.config
                .url_obj
                .path
                .clone()
                .unwrap_or(DEFAULT_HTTP_PATH.to_string())
        } else {
            format!(
                "{}{}{}",
                self.config.url_obj.path.clone().unwrap_or_default(),
                QUERY_SYMBOL,
                query
            )
        };
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
    /// Returns a `Result<HttpResponse, Error>`, where:
    /// - `Ok(HttpResponse)` contains the HTTP response received from the server.
    /// - `Err(Error)` indicates that an error occurred while sending the request or reading the response.
    fn send_get_request(&mut self, stream: &mut Box<dyn ReadWrite>) -> Result<HttpResponse, Error> {
        let mut request: Vec<u8> = Vec::new();
        let path: String = self.get_path();
        let request_line_string: String =
            format!("{} {} {}", Methods::GET, path, self.config.http_version,);
        let request_line: &[u8] = request_line_string.as_bytes();
        request.extend_from_slice(request_line);
        request.extend_from_slice(HTTP_BR_BYTES);
        request.extend_from_slice(&self.get_header_bytes());
        request.extend_from_slice(HTTP_BR_BYTES);
        stream.write_all(&request).unwrap();
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
    /// Returns a `Result<HttpResponse, Error>`, where:
    /// - `Ok(HttpResponse)` contains the HTTP response received from the server.
    /// - `Err(Error)` indicates that an error occurred while sending the request or reading the response.
    fn send_post_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
    ) -> Result<HttpResponse, Error> {
        let mut request: Vec<u8> = Vec::new();
        let path: String = self.get_path();
        let request_line_string: String =
            format!("{} {} {}", Methods::POST, path, self.config.http_version,);
        let request_line: &[u8] = request_line_string.as_bytes();
        request.extend_from_slice(request_line);
        request.extend_from_slice(HTTP_BR_BYTES);
        request.extend_from_slice(&self.get_header_bytes());
        request.extend_from_slice(HTTP_BR_BYTES);
        let body_str: &Vec<u8> = &self.get_body_bytes();
        request.extend_from_slice(body_str);
        stream.write_all(&request).unwrap();
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
    /// Returns a `Result<HttpResponse, Error>`, where:
    /// - `Ok(HttpResponse)` contains the complete HTTP response after processing headers and body.
    /// - `Err(Error)` indicates that an error occurred while reading the response.
    fn read_response(&mut self, stream: &mut Box<dyn ReadWrite>) -> Result<HttpResponse, Error> {
        let buffer_size: usize = self.config.buffer;
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        let mut response_bytes: Vec<u8> = Vec::new();
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<Vec<u8>> = None;
        let http_version: String = self.config.http_version.to_string();
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
        self.response = HttpResponse::from(&response_bytes);
        if !self.config.redirect || redirect_url.is_none() {
            return Ok(self.response.clone());
        }
        let url: String =
            String::from_utf8(redirect_url.unwrap()).map_err(|_| Error::InvalidUrl)?;
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
    /// Returns `Ok(HttpResponse)` if the redirection is successful, or `Err(Error)` otherwise.
    fn handle_redirect(&mut self, url: String) -> Result<HttpResponse, Error> {
        if self.tmp.visit_url.contains(&url) {
            return Err(Error::RedirectUrlDeadLoop);
        }
        self.tmp.visit_url.insert(url.clone());
        if self.config.redirect && self.config.redirect_times >= self.config.max_redirect_times {
            return Err(Error::MaxRedirectTimes);
        }
        self.config.redirect_times = self.config.redirect_times + 1;
        self.url(url.clone());
        self.send()
    }

    /// Determines the appropriate port for the HTTP request.
    ///
    /// # Parameters
    ///
    /// - `port`: The default port (if any).
    ///
    /// Returns the resolved port.
    fn get_port(&self, port: u16) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol: Protocol = self.get_protocol();
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
    /// - `Err(Error)`: An error indicating what went wrong during the connection process.
    ///
    /// # Errors
    ///
    /// - `Error::TlsConnectorBuildError`: If the TLS connector could not be built.
    /// - `Error::TcpStreamConnectError`: If the TCP connection could not be established.
    /// - `Error::SetReadTimeoutError`: If setting the read timeout failed.
    /// - `Error::TlsStreamConnectError`: If the TLS stream could not be established.
    fn get_connection_stream(&self, host: String, port: u16) -> Result<Box<dyn ReadWrite>, Error> {
        let host_port: (String, u16) = (host.clone(), port);
        let timeout: Duration = Duration::from_millis(self.config.timeout);
        let stream: Result<Box<dyn ReadWrite>, Error> = if self.get_protocol().is_https() {
            let tls_connector: TlsConnector = TlsConnector::builder()
                .build()
                .map_err(|_| Error::TlsConnectorBuildError)?;
            let tcp_stream: TcpStream =
                TcpStream::connect(host_port.clone()).map_err(|_| Error::TcpStreamConnectError)?;
            tcp_stream
                .set_read_timeout(Some(timeout))
                .map_err(|_| Error::SetReadTimeoutError)?;
            tcp_stream
                .set_write_timeout(Some(timeout))
                .map_err(|_| Error::SetWriteTimeoutError)?;
            let tls_stream: TlsStream<TcpStream> = tls_connector
                .connect(&host.clone(), tcp_stream)
                .map_err(|_| Error::TlsStreamConnectError)?;
            Ok(Box::new(tls_stream))
        } else {
            let tcp_stream: TcpStream =
                TcpStream::connect(host_port.clone()).map_err(|_| Error::TcpStreamConnectError)?;
            tcp_stream
                .set_read_timeout(Some(timeout))
                .map_err(|_| Error::SetReadTimeoutError)?;
            tcp_stream
                .set_write_timeout(Some(timeout))
                .map_err(|_| Error::SetWriteTimeoutError)?;
            Ok(Box::new(tcp_stream))
        };
        stream
    }

    /// Sends the HTTP request.
    ///
    /// Determines the HTTP method and constructs the appropriate request (GET or POST).
    ///
    /// Returns `Ok(HttpResponse)` if the request is successful, or `Err(Error)` otherwise.
    pub fn send(&mut self) -> Result<HttpResponse, Error> {
        self.config.url_obj = self.parse_url().map_err(|_| Error::InvalidUrl)?;
        let methods: Methods = self.get_methods();
        let host: String = self.config.url_obj.host.clone().unwrap_or_default();
        let port: u16 = self.get_port(self.config.url_obj.port.clone().unwrap_or_default());
        let mut stream: Box<dyn ReadWrite> = self
            .get_connection_stream(host, port)
            .map_err(|_| Error::TcpStreamConnectError)?;
        let res: Result<HttpResponse, Error> = match methods {
            m if m.is_get() => self.send_get_request(&mut stream),
            m if m.is_post() => self.send_post_request(&mut stream),
            _ => Err(Error::RequestError),
        };
        res
    }
}

/// Default implementation for `HttpRequest`.
impl Default for HttpRequest {
    fn default() -> HttpRequest {
        HttpRequest {
            methods: Arc::new(Methods::new()),
            url: Arc::new(String::new()),
            header: Arc::new(HashMap::new()),
            body: Arc::new(Body::default()),
            config: Config::default(),
            tmp: Tmp::default(),
            response: HttpResponse::default(),
        }
    }
}
