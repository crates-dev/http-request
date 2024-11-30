use native_tls::{TlsConnector, TlsStream};

use super::r#type::HttpRequest;
use crate::{
    constant::http::{CONTENT_TYPE, HTTP_BR, HTTP_DOUBLE_BR},
    content_type::r#type::ContentType,
    methods::r#type::Methods,
    protocol::r#type::Protocol,
    request::error::Error,
    request_url::r#type::RequestUrl,
};
use crate::{
    constant::{
        http::{
            CONNECTION, CONTENT_LENGTH, DEFAULT_HTTP_PATH, DEFAULT_HTTP_VERSION, HOST, LOCATION,
        },
        request::DEFAULT_TIMEOUT,
    },
    global_trait::r#trait::ReadWrite,
    global_type::r#type::{Body, Header},
    response::r#type::HttpResponse,
};
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
        self.protocol.as_ref().clone()
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
        let header: HashMap<String, String> = self.get_header();
        let mut header_string: String = String::new();
        for (key, value) in header {
            let line: String = format!("{}: {}{}", key, value, HTTP_BR);
            header_string.push_str(&line);
        }
        header_string
    }

    /// Converts the body into a URL-encoded string.
    ///
    /// Returns a string in the format `key1=value1&key2=value2`.
    fn get_body_str(&self) -> String {
        let content_type_key: String = CONTENT_TYPE.to_lowercase();
        let header: HashMap<String, String> = self.get_header();
        let body: HashMap<String, String> = self.get_body();
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

    /// Sends an HTTP GET request to the specified URL through the given stream.
    ///
    /// This method constructs an HTTP GET request based on the provided `RequestUrl` object,
    /// including necessary headers, and sends it through the provided stream. It then reads
    /// the HTTP response from the stream and returns it.
    ///
    /// # Parameters
    ///
    /// - `stream`: A mutable reference to a stream that implements the `ReadWrite` trait.
    ///   This stream is used to send the HTTP request and receive the HTTP response.
    /// - `url_obj`: A reference to a `RequestUrl` object that contains the URL information
    ///   (such as the host and path) to be used in the GET request.
    ///
    /// # Returns
    ///
    /// - `HttpResponse`: The HTTP response that was received after sending the GET request.
    ///   This contains the status code, headers, and body of the response.
    ///
    /// # Errors
    ///
    /// This function may panic if the request construction or writing to the stream fails,
    /// or if reading the response encounters an issue. It is recommended to handle potential
    /// errors more gracefully in production code.
    fn send_get_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
        url_obj: &RequestUrl,
    ) -> HttpResponse {
        let path: String = url_obj.path.clone().unwrap_or("/".to_string());
        let mut request: String = format!(
            "{} {} {}{}",
            Methods::GET,
            path,
            DEFAULT_HTTP_VERSION,
            HTTP_BR
        );
        request.push_str(&format!(
            "{}: {}{}",
            HOST,
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        request.push_str(HTTP_BR);
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    /// Sends an HTTP POST request to the specified URL through the given stream.
    ///
    /// This method constructs an HTTP POST request, including the body, headers, and other
    /// necessary details, and sends it through the provided stream. After sending the request,
    /// it reads the HTTP response from the stream and returns it.
    ///
    /// # Parameters
    ///
    /// - `stream`: A mutable reference to a stream that implements the `ReadWrite` trait.
    ///   This stream is used to send the HTTP request and receive the HTTP response.
    /// - `url_obj`: A reference to a `RequestUrl` object that contains the URL information
    ///   (such as the host and path) to be used in the POST request.
    ///
    /// # Returns
    ///
    /// - `HttpResponse`: The HTTP response that was received after sending the POST request.
    ///   This contains the status code, headers, and body of the response.
    ///
    /// # Errors
    ///
    /// This function may panic if the request construction or writing to the stream fails,
    /// or if reading the response encounters an issue. It is recommended to handle potential
    /// errors more gracefully in production code.
    fn send_post_request(
        &mut self,
        stream: &mut Box<dyn ReadWrite>,
        url_obj: &RequestUrl,
    ) -> HttpResponse {
        let path: String = url_obj
            .path
            .clone()
            .unwrap_or(DEFAULT_HTTP_PATH.to_string());
        let mut request: String = format!(
            "{} {} {}{}",
            Methods::POST,
            path,
            DEFAULT_HTTP_VERSION,
            HTTP_BR
        );
        request.push_str(&format!(
            "{}: {}{}",
            HOST,
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        let body_str: String = self.get_body_str();
        request.push_str(&format!(
            "{}: {}{}",
            CONTENT_LENGTH,
            body_str.len(),
            HTTP_BR
        ));
        request.push_str(HTTP_BR);
        request.push_str(&format!("{}{}", body_str, HTTP_BR));
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    /// Reads the HTTP response from the given stream and handles potential redirects.
    ///
    /// This method reads data from the provided stream and processes the response. It handles
    /// the response headers, including checking for redirects (HTTP status codes 3xx), and
    /// collects the response body once the headers are fully received. If a redirect is detected,
    /// it follows the redirect URL and retrieves the new response.
    ///
    /// # Parameters
    ///
    /// - `stream`: A mutable reference to a stream that implements the `ReadWrite` trait.
    ///   This stream is used to read the HTTP response from the server.
    ///
    /// # Returns
    ///
    /// - `HttpResponse`: The final HTTP response after reading from the stream, including the
    ///   status code, headers, and body. If a redirect is encountered, the response from the
    ///   redirected URL is returned.
    ///
    /// # Errors
    ///
    /// This function may panic if unexpected issues occur during reading or processing the
    /// response. In production code, error handling should be implemented to manage any failures
    /// during the reading or redirect process.
    fn read_response(&mut self, stream: &mut Box<dyn ReadWrite>) -> HttpResponse {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut response_string: String = String::new();
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<String> = None;
        'first_loop: while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }
            response_string.push_str(&String::from_utf8_lossy(&buffer[..n]));
            if !headers_done && response_string.contains(HTTP_DOUBLE_BR) {
                headers_done = true;
                let status_pos_res: Option<usize> = response_string.find(DEFAULT_HTTP_VERSION);
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
                    let end_pos_res = response_string[start..].find(HTTP_BR);
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
        let res: HttpResponse = redirect_url
            .and_then(|url| {
                // Handle redirects.
                let redirect_res: HttpResponse = if let Ok(res) = self.handle_redirect(url) {
                    res
                } else {
                    response.clone()
                };
                Some(redirect_res)
            })
            .unwrap_or_else(|| response);
        res
    }

    /// Handles HTTP redirects by following the redirection URL.
    ///
    /// # Arguments
    ///
    /// - `url`: The redirection URL to follow.
    ///
    /// Returns `Ok(HttpResponse)` if the redirection is successful, or `Err(Error)` otherwise.
    fn handle_redirect(&mut self, url: String) -> Result<HttpResponse, Error> {
        self.url(url.clone());
        let url_obj: RequestUrl = self.parse_url().map_err(|_| Error::InvalidUrl)?;
        let host: String = url_obj.host.unwrap_or_default();
        let port: u16 = self.get_port(url_obj.port.clone().unwrap_or_default());
        let path: String = url_obj.path.unwrap_or_default();
        let request: String = format!(
            "{} {} {}{}{}: {}{}{}: close{}",
            Methods::GET,
            path,
            DEFAULT_HTTP_VERSION,
            HTTP_DOUBLE_BR,
            HOST,
            host,
            HTTP_DOUBLE_BR,
            CONNECTION,
            HTTP_DOUBLE_BR
        );
        let mut stream = self
            .get_connection_stream(host, port)
            .map_err(|_| Error::TcpStreamConnectError)?;
        stream.write_all(request.as_bytes()).unwrap();
        Ok(self.read_response(&mut stream))
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
        let stream: Result<Box<dyn ReadWrite>, Error> = if self.get_protocol().is_https() {
            let tls_connector: TlsConnector = TlsConnector::builder()
                .build()
                .map_err(|_| Error::TlsConnectorBuildError)?;
            let tcp_stream: TcpStream =
                TcpStream::connect(host_port.clone()).map_err(|_| Error::TcpStreamConnectError)?;
            tcp_stream
                .set_read_timeout(Some(Duration::from_secs(*self.timeout)))
                .map_err(|_| Error::SetReadTimeoutError)?;
            let tls_stream: TlsStream<TcpStream> = tls_connector
                .connect(&host.clone(), tcp_stream)
                .map_err(|_| Error::TlsStreamConnectError)?;
            Ok(Box::new(tls_stream))
        } else {
            let tcp_stream: TcpStream =
                TcpStream::connect(host_port.clone()).map_err(|_| Error::TcpStreamConnectError)?;
            tcp_stream
                .set_read_timeout(Some(Duration::from_millis(*self.timeout)))
                .map_err(|_| Error::SetReadTimeoutError)?;
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
        let url_obj: RequestUrl = self.parse_url().map_err(|_| Error::InvalidUrl)?;
        let methods: Methods = self.get_methods();
        let host: String = url_obj.host.clone().unwrap_or_default();
        let port: u16 = self.get_port(url_obj.port.clone().unwrap_or_default());
        let mut stream: Box<dyn ReadWrite> = self
            .get_connection_stream(host, port)
            .map_err(|_| Error::TcpStreamConnectError)?;
        let res: Result<HttpResponse, Error> = match methods {
            m if m.is_get() => Ok(self.send_get_request(&mut stream, &url_obj)),
            m if m.is_post() => Ok(self.send_post_request(&mut stream, &url_obj)),
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
            protocol: Arc::new(Protocol::new()),
            header: Arc::new(HashMap::new()),
            body: Arc::new(HashMap::new()),
            timeout: Arc::new(DEFAULT_TIMEOUT),
        }
    }
}
