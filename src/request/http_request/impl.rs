use super::r#type::HttpRequest;
use crate::{
    body::r#type::Body,
    constant::{
        common::APP_NAME,
        http::{
            ACCEPT, ACCEPT_VALUE, CONTENT_TYPE, HTTP_BR, HTTP_DOUBLE_BR, QUERY_SYMBOL, USER_AGENT,
        },
    },
    content_type::r#type::ContentType,
    methods::r#type::Methods,
    protocol::r#type::Protocol,
    request::{
        config::r#type::Config, error::Error, request_url::r#type::RequestUrl, tmp::r#type::Tmp,
    },
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
    /// # Arguments
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

    /// Converts the headers into a formatted HTTP header string.
    ///
    /// Returns a string where each header is formatted as `key: value`.
    fn get_header_str(&self) -> String {
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
                    &self.get_body_str().len().to_string()
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
        header_string
    }

    /// Converts the body into a URL-encoded string.
    ///
    /// Returns a string in the format `key1=value1&key2=value2`.
    fn get_body_str(&self) -> String {
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
        res
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
        let mut request: String = format!(
            "{} {} {}{}",
            Methods::GET,
            path,
            self.config.http_version,
            HTTP_BR
        );
        request.push_str(&self.get_header_str());
        request.push_str(HTTP_BR);
        stream.write_all(request.as_bytes()).unwrap();
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
        let path: String = self
            .config
            .url_obj
            .path
            .clone()
            .unwrap_or(DEFAULT_HTTP_PATH.to_string());
        let mut request: String = format!(
            "{} {} {}{}",
            Methods::POST,
            path,
            self.config.http_version,
            HTTP_BR
        );
        request.push_str(&self.get_header_str());
        request.push_str(HTTP_BR);
        let body_str: &String = &self.get_body_str();
        request.push_str(body_str);
        stream.write_all(request.as_bytes()).unwrap();
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
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut response_string: String = String::new();
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<String> = None;
        let http_version: String = self.config.http_version.to_string();
        'first_loop: while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }
            response_string.push_str(&String::from_utf8_lossy(&buffer[..n]));
            if !headers_done && response_string.contains(HTTP_DOUBLE_BR) {
                headers_done = true;
                let status_pos_res: Option<usize> = response_string.find(&http_version);
                if status_pos_res.is_none() {
                    continue;
                }
                let status_pos: usize = status_pos_res.unwrap_or_default();
                let status_code: usize = response_string[status_pos + 9..status_pos + 12]
                    .trim()
                    .parse::<usize>()
                    .unwrap_or_default();
                if (300..=399).contains(&status_code) {
                    let location_sign_key: String = format!("{}:", LOCATION.to_lowercase());
                    let location_pos_res = response_string.to_lowercase().find(&location_sign_key);
                    if location_pos_res.is_none() {
                        continue;
                    }
                    let location_pos: usize = location_pos_res.unwrap_or_default();
                    let start: usize = location_pos + location_sign_key.len();
                    let end_pos_res: Option<usize> = response_string[start..].find(HTTP_BR);
                    if end_pos_res.is_none() {
                        continue;
                    }
                    let end_pos: usize = end_pos_res.unwrap_or_default();
                    redirect_url = Some(response_string[start..start + end_pos].trim().to_string());
                }
                content_length = HttpResponse::get_content_length(&response_string);
            }
            if headers_done && response_string.len() >= content_length {
                break 'first_loop;
            }
        }
        let response: HttpResponse = HttpResponse::from(&response_string);
        if redirect_url.is_none() {
            return Ok(response);
        }
        let url: String = redirect_url.unwrap();
        self.handle_redirect(url)
    }

    /// Handles HTTP redirects by following the redirection URL.
    ///
    /// # Arguments
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
    /// # Arguments
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
        }
    }
}
