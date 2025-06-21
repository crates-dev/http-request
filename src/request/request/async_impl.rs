use crate::*;

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
            let n: usize = stream
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

        let (should_redirect, should_decode, buffer_size) = {
            if let Ok(config) = self.config.read() {
                (config.redirect, config.decode, config.buffer)
            } else {
                (false, false, DEFAULT_BUFFER_SIZE)
            }
        };

        if !should_redirect || redirect_url.is_none() {
            if should_decode {
                if let Ok(mut response) = self.response.write() {
                    *response = response.decode(buffer_size);
                }
            }
            return Ok(Box::new(
                self.response
                    .read()
                    .map_or(HttpResponseBinary::default(), |response| response.clone()),
            ));
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
            {
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
        let config: Config = self
            .config
            .read()
            .map_or(Config::default(), |config| config.clone());
        if let Some(proxy_config) = &config.proxy {
            return self
                .get_proxy_connection_stream_async(host, port, proxy_config)
                .await;
        }
        let host_port: (String, u16) = (host.clone(), port);
        let tcp_stream: AsyncTcpStream = AsyncTcpStream::connect(host_port.clone())
            .await
            .map_err(|err| RequestError::TcpStreamConnect(err.to_string()))?;

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

            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
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

    /// Establishes an async proxy connection stream to the specified host and port.
    async fn get_proxy_connection_stream_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        match proxy_config.proxy_type {
            ProxyType::Http | ProxyType::Https => {
                self.get_http_proxy_connection_async(target_host, target_port, proxy_config)
                    .await
            }
            ProxyType::Socks5 => {
                self.get_socks5_proxy_connection_async(target_host, target_port, proxy_config)
                    .await
            }
        }
    }

    /// Establishes an async HTTP/HTTPS proxy connection.
    async fn get_http_proxy_connection_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let tcp_stream: AsyncTcpStream = AsyncTcpStream::connect(proxy_host_port)
            .await
            .map_err(|err| RequestError::TcpStreamConnect(err.to_string()))?;

        let mut proxy_stream: BoxAsyncReadWrite = if proxy_config.proxy_type == ProxyType::Https {
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

            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(proxy_config.host.clone())
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
            let tls_stream: TlsStream<AsyncTcpStream> = connector
                .connect(dns_name, tcp_stream)
                .await
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
            Box::new(tls_stream)
        } else {
            Box::new(tcp_stream)
        };

        let connect_request: String = if let (Some(username), Some(password)) =
            (&proxy_config.username, &proxy_config.password)
        {
            let auth: String = format!("{}:{}", username, password);
            let auth_encoded: String = Self::base64_encode(auth.as_bytes());
            format!(
                "CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\nProxy-Authorization: Basic {}\r\n\r\n",
                target_host, target_port, target_host, target_port, auth_encoded
            )
        } else {
            format!(
                "CONNECT {}:{} HTTP/1.1\r\nHost: {}:{}\r\n\r\n",
                target_host, target_port, target_host, target_port
            )
        };

        proxy_stream
            .write_all(connect_request.as_bytes())
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;
        proxy_stream
            .flush()
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;
        let mut response_buffer: [u8; 1024] = [0u8; 1024];
        let bytes_read = proxy_stream
            .read(&mut response_buffer)
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;
        let response: Cow<'_, str> = String::from_utf8_lossy(&response_buffer[..bytes_read]);
        if !response.starts_with("HTTP/1.1 200") && !response.starts_with("HTTP/1.0 200") {
            return Err(RequestError::Request(format!(
                "Proxy connection failed: {}",
                response.lines().next().unwrap_or("Unknown error")
            )));
        }

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

            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(target_host.clone())
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;

            let tunnel_stream = crate::request::proxy_tunnel::ProxyTunnelStream::new(proxy_stream);
            let tls_stream: TlsStream<crate::request::proxy_tunnel::ProxyTunnelStream> = connector
                .connect(dns_name, tunnel_stream)
                .await
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
            return Ok(Box::new(tls_stream));
        }

        Ok(proxy_stream)
    }

    /// Establishes an async SOCKS5 proxy connection.
    async fn get_socks5_proxy_connection_async(
        &self,
        target_host: String,
        target_port: u16,
        proxy_config: &ProxyConfig,
    ) -> Result<BoxAsyncReadWrite, RequestError> {
        let proxy_host_port: (String, u16) = (proxy_config.host.clone(), proxy_config.port);
        let mut tcp_stream: AsyncTcpStream = AsyncTcpStream::connect(proxy_host_port)
            .await
            .map_err(|err| RequestError::TcpStreamConnect(err.to_string()))?;

        let auth_methods: Vec<u8> =
            if proxy_config.username.is_some() && proxy_config.password.is_some() {
                vec![0x05, 0x02, 0x00, 0x02]
            } else {
                vec![0x05, 0x01, 0x00]
            };

        tcp_stream
            .write_all(&auth_methods)
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;

        let mut response: [u8; 2] = [0u8; 2];
        tcp_stream
            .read_exact(&mut response)
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;

        if response[0] != 0x05 {
            return Err(RequestError::Request("Invalid SOCKS5 response".to_string()));
        }

        match response[1] {
            0x00 => {}
            0x02 => {
                if let (Some(username), Some(password)) =
                    (&proxy_config.username, &proxy_config.password)
                {
                    let mut auth_request = vec![0x01]; // Version 1
                    auth_request.push(username.len() as u8);
                    auth_request.extend_from_slice(username.as_bytes());
                    auth_request.push(password.len() as u8);
                    auth_request.extend_from_slice(password.as_bytes());

                    tcp_stream
                        .write_all(&auth_request)
                        .await
                        .map_err(|err| RequestError::Request(err.to_string()))?;

                    let mut auth_response = [0u8; 2];
                    tcp_stream
                        .read_exact(&mut auth_response)
                        .await
                        .map_err(|err| RequestError::Request(err.to_string()))?;

                    if auth_response[1] != 0x00 {
                        return Err(RequestError::Request(
                            "SOCKS5 authentication failed".to_string(),
                        ));
                    }
                } else {
                    return Err(RequestError::Request(
                        "SOCKS5 proxy requires authentication".to_string(),
                    ));
                }
            }
            0xFF => {
                return Err(RequestError::Request(
                    "No acceptable SOCKS5 authentication methods".to_string(),
                ));
            }
            _ => {
                return Err(RequestError::Request(
                    "Unsupported SOCKS5 authentication method".to_string(),
                ));
            }
        }

        let mut connect_request: Vec<u8> = vec![0x05, 0x01, 0x00];

        if target_host.parse::<Ipv4Addr>().is_ok() {
            connect_request.push(0x01);
            let ip: Ipv4Addr = target_host.parse().unwrap();
            connect_request.extend_from_slice(&ip.octets());
        } else if target_host.parse::<Ipv6Addr>().is_ok() {
            connect_request.push(0x04);
            let ip: Ipv6Addr = target_host.parse().unwrap();
            connect_request.extend_from_slice(&ip.octets());
        } else {
            connect_request.push(0x03);
            connect_request.push(target_host.len() as u8);
            connect_request.extend_from_slice(target_host.as_bytes());
        }

        connect_request.extend_from_slice(&target_port.to_be_bytes());

        tcp_stream
            .write_all(&connect_request)
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;

        let mut connect_response: [u8; 4] = [0u8; 4];
        tcp_stream
            .read_exact(&mut connect_response)
            .await
            .map_err(|err| RequestError::Request(err.to_string()))?;

        if connect_response[0] != 0x05 || connect_response[1] != 0x00 {
            return Err(RequestError::Request(format!(
                "SOCKS5 connection failed with code: {}",
                connect_response[1]
            )));
        }

        match connect_response[3] {
            0x01 => {
                let mut skip: [u8; 6] = [0u8; 6];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|err| RequestError::Request(err.to_string()))?;
            }
            0x03 => {
                let mut len: [u8; 1] = [0u8; 1];
                tcp_stream
                    .read_exact(&mut len)
                    .await
                    .map_err(|err| RequestError::Request(err.to_string()))?;
                let mut skip: Vec<u8> = vec![0u8; len[0] as usize + 2];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|err| RequestError::Request(err.to_string()))?;
            }
            0x04 => {
                let mut skip: [u8; 18] = [0u8; 18];
                tcp_stream
                    .read_exact(&mut skip)
                    .await
                    .map_err(|err| RequestError::Request(err.to_string()))?;
            }
            _ => {
                return Err(RequestError::Request(
                    "Invalid SOCKS5 address type".to_string(),
                ));
            }
        }

        let proxy_stream: BoxAsyncReadWrite = Box::new(tcp_stream);

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

            let tls_config: ClientConfig = ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            let connector: TlsConnector = TlsConnector::from(Arc::new(tls_config));
            let dns_name: ServerName<'_> = ServerName::try_from(target_host.clone())
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;

            let tunnel_stream = crate::request::proxy_tunnel::ProxyTunnelStream::new(proxy_stream);
            let tls_stream: TlsStream<crate::request::proxy_tunnel::ProxyTunnelStream> = connector
                .connect(dns_name, tunnel_stream)
                .await
                .map_err(|err| RequestError::TlsConnectorBuild(err.to_string()))?;
            return Ok(Box::new(tls_stream));
        }

        Ok(proxy_stream)
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
