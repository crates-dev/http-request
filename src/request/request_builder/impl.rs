use crate::*;

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            http_request: HttpRequest::default(),
            builder: HttpRequest::default(),
        }
    }
}

impl RequestBuilder {
    /// Creates a new instance of the builder with default values.
    ///
    /// This method initializes the `RequestBuilder` with default values for all
    /// fields.
    ///
    /// # Returns
    /// Returns a new instance of `RequestBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the HTTP method for the request.
    ///
    /// This method allows you to specify the HTTP method (e.g., GET, POST) for the
    /// request being built.
    ///
    /// # Parameters
    /// - `methods`: The HTTP method to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `RequestBuilder` to allow method chaining.
    pub fn post(&mut self, url: &str) -> &mut Self {
        self.http_request.methods = Arc::new(Method::POST);
        self.url(url);
        self
    }

    /// Sets the HTTP method for the request.
    ///
    /// This method allows you to specify the HTTP method (e.g., GET, POST) for the
    /// request being built.
    ///
    /// # Parameters
    /// - `methods`: The HTTP method to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `RequestBuilder` to allow method chaining.
    pub fn get(&mut self, url: &str) -> &mut Self {
        self.http_request.methods = Arc::new(Method::GET);
        self.url(url);
        self
    }

    /// Sets the target URL of the request.
    ///
    /// This method allows you to specify the URL for the request being built.
    ///
    /// # Parameters
    /// - `url`: The target URL of the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `RequestBuilder` to allow method chaining.
    fn url(&mut self, url: &str) -> &mut Self {
        self.http_request.url = Arc::new(url.to_owned());
        self
    }

    /// Sets the HTTP version to 1.1 for the request configuration.
    ///
    /// This method updates the HTTP version to `HTTP1_1` for the current
    /// `http_request` configuration. It allows the user to force the
    /// request to use HTTP 1.1 only, overriding any other version that may
    /// have been previously set.
    ///
    /// # Returns
    /// Returns a mutable reference to `self` to allow method chaining.
    pub fn http1_1_only(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.http_version = HttpVersion::HTTP1_1;
        }
        self
    }

    /// Sets the HTTP version to 2.0 for the request configuration.
    ///
    /// This method updates the HTTP version to `HTTP2` for the current
    /// `http_request` configuration. It allows the user to force the
    /// request to use HTTP 2.0 only, overriding any other version that may
    /// have been previously set.
    ///
    /// # Returns
    /// Returns a mutable reference to `self` to allow method chaining.
    pub fn http2_only(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.http_version = HttpVersion::HTTP2;
        }
        self
    }

    /// Sets the headers for the request.
    ///
    /// This method allows you to specify the headers for the request being built.
    /// Existing headers may be merged with the provided ones.
    ///
    /// # Parameters
    /// - `header`: The headers to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `RequestBuilder` to allow method chaining.
    pub fn headers<K, V>(&mut self, header: HashMapXxHash3_64<K, V>) -> &mut Self
    where
        K: ToString,
        V: ToString,
    {
        if let Some(tmp_header) = Arc::get_mut(&mut self.http_request.header) {
            for (key, value) in header {
                tmp_header.insert(key.to_string(), value.to_string());
            }
        }
        self
    }

    /// Sets the JSON body of the request.
    ///
    /// This method allows you to set the body of the request as JSON data. If there is existing
    /// body data, it will be replaced with the provided JSON data.
    ///
    /// # Parameters
    /// - `body`: The JSON body data to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `RequestBuilder` to allow method chaining.
    pub fn json(&mut self, body: JsonValue) -> &mut Self {
        if let JsonValue::Object(map) = body {
            let mut res_body: HashMapXxHash3_64<String, JsonValue> = hash_map_xx_hash3_64();
            for (k, v) in map.iter() {
                res_body.insert(k.to_string(), v.clone());
            }
            self.http_request.body = Arc::new(Body::Json(res_body));
        }
        self
    }

    /// Sets the text body of the request.
    ///
    /// This method allows you to set the body of the request as plain text. If there is existing
    /// body data, it will be replaced with the provided text data.
    ///
    /// # Parameters
    /// - `body`: The text body data to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `RequestBuilder` to allow method chaining.
    pub fn text<T: ToString>(&mut self, body: T) -> &mut Self {
        self.http_request.body = Arc::new(Body::Text(body.to_string()));
        self
    }

    /// Sets the HTTP request body to the given binary content.
    ///
    /// This method assigns the provided binary data to the body of the HTTP request.
    /// The body is wrapped in an `Arc` to enable shared ownership and safe concurrency.
    ///
    /// # Parameters
    ///
    /// - `body` - A `BodyBinary` representing the binary content to be used as the HTTP request body.
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the current instance of the struct, allowing method chaining.
    ///
    /// # Notes
    ///
    /// This method overrides any previously set body. Use it when you want to assign binary content
    /// specifically as the body of the HTTP request.
    pub fn body<T: Into<Vec<u8>>>(&mut self, body: T) -> &mut Self {
        self.http_request.body = Arc::new(Body::Binary(body.into()));
        self
    }

    /// Sets the timeout value for the current connection.
    ///
    /// This method sets the timeout duration for the connection, which is used to determine
    /// how long the system should wait for a response before considering the connection attempt
    /// as failed. The timeout value is stored in an `Arc` to allow it to be shared safely across
    /// multiple threads if needed.
    ///
    /// # Parameters
    ///
    /// - `timeout`: The timeout duration in seconds. This value will be used to configure the
    ///   connection timeout.
    ///
    /// # Returns
    /// Returns a mutable reference to the `RequestBuilder` to allow method chaining.
    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.timeout = timeout;
        }
        self
    }

    /// Enables HTTP redirection for the request.
    ///
    /// This method sets the `redirect` property of the `http_request` to `true`.
    /// It returns a mutable reference to the current instance, allowing method chaining.
    pub fn redirect(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.redirect = true;
        }
        self
    }

    /// Unenables HTTP redirection for the request.
    ///
    /// This method sets the `redirect` property of the `http_request` to `false`.
    /// It returns a mutable reference to the current instance, allowing method chaining.
    pub fn unredirect(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.redirect = false;
        };
        self
    }

    /// Sets the maximum number of allowed redirections for the HTTP request.
    ///
    /// This method updates the `max_redirect_times` field in the configuration and returns a mutable
    /// reference to `self` to enable method chaining.
    ///
    /// # Parameters
    ///
    /// - `num` - The maximum number of redirections allowed. A value of `0` disables redirection.
    ///
    /// # Returns
    ///
    /// A mutable reference to the current instance for method chaining.
    ///
    /// # Notes
    ///
    /// Ensure that the value provided to `num` is within a valid range. Excessively high values
    /// may lead to performance issues or unintended behavior.
    pub fn max_redirect_times(&mut self, num: usize) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.max_redirect_times = num;
        }
        self
    }

    /// Sets the buffer size for the HTTP request configuration.
    ///
    /// This method allows you to set the size of the buffer used for reading
    /// the HTTP response. It modifies the `buffer` field of the HTTP request's
    /// configuration, which will be used when processing the response data.
    ///
    /// # Parameters
    /// - `buffer`: The size of the buffer to be used, in bytes.
    ///
    /// # Returns
    /// Returns a mutable reference to `self`, allowing for method chaining.
    pub fn buffer(&mut self, buffer: usize) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.buffer = buffer;
        }
        self
    }

    /// Enables automatic response decoding.
    ///
    /// When enabled, the response body will be automatically decompressed if it is encoded
    /// using a supported compression format (e.g., `gzip`, `deflate`, `br`).
    ///
    /// # Returns
    /// A mutable reference to the current instance, allowing for method chaining.
    pub fn decode(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.decode = true;
        }
        self
    }

    /// Disables automatic response decoding.
    ///
    /// When disabled, the response body will not be automatically decompressed,
    /// and the raw encoded data will be returned as-is.
    ///
    /// # Returns
    /// A mutable reference to the current instance, allowing for method chaining.
    pub fn undecode(&mut self) -> &mut Self {
        if let Ok(mut config) = self.http_request.config.write() {
            config.decode = false;
        }
        self
    }

    /// Finalizes the builder and returns a fully constructed async `HttpRequest` instance.
    ///
    /// This method takes the current configuration stored in `http_request`, creates a new
    /// `HttpRequest` instance with the configuration, and resets the builder's temporary
    /// state for further use.
    ///
    /// # Returns
    /// Returns a fully constructed `BoxAsyncRequestTrait` instance based on the current builder state.
    pub fn build_async(&mut self) -> BoxAsyncRequestTrait {
        self.builder = self.http_request.clone();
        self.http_request = HttpRequest::default();
        Box::new(self.builder.clone())
    }

    /// Finalizes the builder and returns a fully constructed `HttpRequest` instance.
    ///
    /// This method takes the current configuration stored in `http_request`, creates a new
    /// `HttpRequest` instance with the configuration, and resets the builder's temporary
    /// state for further use.
    ///
    /// # Returns
    /// Returns a fully constructed `BoxRequestTrait` instance based on the current builder state.
    pub fn build(&mut self) -> BoxRequestTrait {
        self.builder = self.http_request.clone();
        self.http_request = HttpRequest::default();
        Box::new(self.builder.clone())
    }
}
