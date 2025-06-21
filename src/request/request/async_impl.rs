use crate::*;
use std::future::Future;
use std::pin::Pin;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as AsyncTcpStream;
use tokio_rustls::{TlsConnector, client::TlsStream};

/// Async implementation for HttpRequest
impl HttpRequest {
    /// Sends an async GET request over the provided stream and returns the HTTP response.
    async fn send_get_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
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
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;

        self.read_response_async(stream).await
    }

    /// Sends an async POST request over the provided stream and returns the HTTP response.
    async fn send_post_request_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
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
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;
        stream
            .flush()
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;

        self.read_response_async(stream).await
    }

    /// Reads an async HTTP response from the provided stream.
    async fn read_response_async(
        &mut self,
        stream: &mut BoxAsyncReadWrite,
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

        'read_loop: loop {
            let n = stream
                .read(&mut buffer)
                .await
                .map_err(|err| RequestError::Request(err.to_string()))?;

            if n == 0 {
                break;
            }

            if response_bytes.len() + n > response_bytes.capacity() {
                let current_cap: usize = response_bytes.capacity();
                let needed_cap: usize = response_bytes.len() + n;
                let new_capacity: usize = if current_cap == 0 {
                    needed_cap.max(1024)
                } else if needed_cap <= current_cap * 2 {
                    current_cap * 2
                } else {
                    (needed_cap * 3) / 2
                };
                response_bytes.reserve(new_capacity - current_cap);
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
        self.handle_redirect_async(url).await
    }

    /// Handles HTTP redirects asynchronously by following the redirection URL.
    fn handle_redirect_async(
        &mut self,
        url: String,
    ) -> Pin<Box<dyn Future<Output = Result<BoxResponseTrait, RequestError>> + Send + '_>> {
        Box::pin(async move {
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
            self.send_async().await
        })
    }

    /// Establishes an async connection stream to the specified host and port.
    async fn get_connection_stream_async(
        &self,
        host: String,
        port: u16,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let host_port: (String, u16) = (host.clone(), port);
        let tcp_stream: AsyncTcpStream = AsyncTcpStream::connect(host_port.clone())
            .await
            .map_err(|err| RequestError::TcpStreamConnect(err.to_string()))?;

        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());

        if Self::get_protocol(&config).is_https() {
            let roots: RootCertStore = {
                match self.tmp.clone().read() {
                    Ok(tmp) => tmp.root_cert.clone(),
                    Err(err) => {
                        return Err(RequestError::Unknown(format!(
                            "error reading temporary configuration: {}",
                            err
                        )));
                    }
                }
            };

            let config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector = TlsConnector::from(Arc::new(config));
            let dns_name: ServerName<'_> = ServerName::try_from(host.clone())
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
            let tls_stream: TlsStream<AsyncTcpStream> = connector
                .connect(dns_name, tcp_stream)
                .await
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
            Ok(Box::new(tls_stream))
        } else {
            Ok(Box::new(tcp_stream))
        }
    }

    /// Sends the HTTP request asynchronously.
    pub async fn send_async(&mut self) -> RequestResult {
        let methods: Method = self.get_methods();
        let (host, port) = {
            if let Ok(mut config) = self.config.write() {
                config.url_obj = self
                    .parse_url()
                    .map_err(|err| RequestError::InvalidUrl(err.to_string()))?;
                let host = config.url_obj.host.clone().unwrap_or_default();
                let port = self.get_port(config.url_obj.port.clone().unwrap_or_default(), &config);
                (host, port)
            } else {
                (String::new(), 0u16)
            }
        };

        let mut stream: BoxAsyncReadWrite = self.get_connection_stream_async(host, port).await?;

        let res: Result<BoxResponseTrait, RequestError> = match methods {
            m if m.is_get() => self.send_get_request_async(&mut stream).await,
            m if m.is_post() => self.send_post_request_async(&mut stream).await,
            err => Err(RequestError::Request(format!(
                "do not support {} method",
                err
            ))),
        };

        res
    }
}

impl AsyncRequestTrait for HttpRequest {
    type RequestResult = RequestResult;

    fn send(&mut self) -> Pin<Box<dyn Future<Output = Self::RequestResult> + Send + '_>> {
        Box::pin(self.send_async())
    }
}
