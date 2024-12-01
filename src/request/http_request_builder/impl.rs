use std::sync::Arc;

use super::r#type::HttpRequestBuilder;
use crate::{
    body::r#type::{Body, BodyJson, BodyText},
    header::r#type::Header,
    methods::r#type::Methods,
    request::{http_request::r#type::HttpRequest, http_version::r#type::HttpVersion},
};

/// Provides a builder pattern implementation for constructing `HttpRequest` instances.
///
/// The `HttpRequestBuilder` struct is used to create and configure `HttpRequest` objects
/// through a series of method calls, enabling a flexible and clear way to construct
/// requests.
///
/// # Traits Implemented
/// - `Default`: Provides a default instance of the builder, initializing all fields
///   with default values.
///
/// # Methods
/// - `new`: Creates a new instance of the builder with default values.
/// - `methods`: Sets the HTTP method for the request (e.g., GET, POST).
/// - `url`: Sets the target URL of the request.
/// - `headers`: Updates the headers of the request. Existing headers may be merged with
///   the provided ones.
/// - `body`: Updates the body of the request. Existing body data may be merged with
///   the provided data.
/// - `builder`: Finalizes the configuration and returns a fully constructed `HttpRequest`
///   instance. Resets the builder's temporary state for subsequent use.
///
/// This builder simplifies the construction of `HttpRequest` objects while maintaining
/// thread safety and ensuring immutability for shared references where applicable.
impl Default for HttpRequestBuilder {
    fn default() -> HttpRequestBuilder {
        HttpRequestBuilder {
            http_request: HttpRequest::default(),
            builder: HttpRequest::default(),
        }
    }
}

impl HttpRequestBuilder {
    /// Creates a new instance of the builder with default values.
    ///
    /// This method initializes the `HttpRequestBuilder` with default values for all
    /// fields.
    ///
    /// # Returns
    /// Returns a new instance of `HttpRequestBuilder`.
    pub fn new() -> Self {
        HttpRequestBuilder::default()
    }

    /// Sets the HTTP method for the request.
    ///
    /// This method allows you to specify the HTTP method (e.g., GET, POST) for the
    /// request being built.
    ///
    /// # Arguments
    /// - `methods`: The HTTP method to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn post(&mut self, url: &str) -> &mut Self {
        self.http_request.methods = Arc::new(Methods::POST);
        self.url(url);
        self
    }

    /// Sets the HTTP method for the request.
    ///
    /// This method allows you to specify the HTTP method (e.g., GET, POST) for the
    /// request being built.
    ///
    /// # Arguments
    /// - `methods`: The HTTP method to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn get(&mut self, url: &str) -> &mut Self {
        self.http_request.methods = Arc::new(Methods::GET);
        self.url(url);
        self
    }

    /// Sets the target URL of the request.
    ///
    /// This method allows you to specify the URL for the request being built.
    ///
    /// # Arguments
    /// - `url`: The target URL of the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
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
        self.http_request.config.http_version = HttpVersion::HTTP1_1;
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
        self.http_request.config.http_version = HttpVersion::HTTP2;
        self
    }

    /// Sets the headers for the request.
    ///
    /// This method allows you to specify the headers for the request being built.
    /// Existing headers may be merged with the provided ones.
    ///
    /// # Arguments
    /// - `header`: The headers to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn headers(&mut self, header: Header) -> &mut Self {
        if let Some(tmp_header) = Arc::get_mut(&mut self.http_request.header) {
            for (key, value) in header {
                tmp_header.insert(key, value);
            }
        }
        self
    }

    /// Sets the JSON body of the request.
    ///
    /// This method allows you to set the body of the request as JSON data. If there is existing
    /// body data, it will be replaced with the provided JSON data.
    ///
    /// # Arguments
    /// - `body`: The JSON body data to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn json(&mut self, body: BodyJson) -> &mut Self {
        self.http_request.body = Arc::new(Body::Json(body));
        self
    }

    /// Sets the text body of the request.
    ///
    /// This method allows you to set the body of the request as plain text. If there is existing
    /// body data, it will be replaced with the provided text data.
    ///
    /// # Arguments
    /// - `body`: The text body data to be set for the request.
    ///
    /// # Returns
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn text(&mut self, body: BodyText) -> &mut Self {
        self.http_request.body = Arc::new(Body::Text(body));
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
    /// Returns a mutable reference to the `HttpRequestBuilder` to allow method chaining.
    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        self.http_request.config.timeout = timeout;
        self
    }

    /// Enables HTTP redirection for the request.
    ///
    /// This method sets the `redirect` property of the `http_request` to `true`.
    /// It returns a mutable reference to the current instance, allowing method chaining.
    pub fn redirect(&mut self) -> &mut Self {
        self.http_request.config.redirect = true;
        self
    }

    /// Unenables HTTP redirection for the request.
    ///
    /// This method sets the `redirect` property of the `http_request` to `false`.
    /// It returns a mutable reference to the current instance, allowing method chaining.
    pub fn unredirect(&mut self) -> &mut Self {
        self.http_request.config.redirect = false;
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
        self.http_request.config.max_redirect_times = num;
        self
    }

    /// Finalizes the builder and returns a fully constructed `HttpRequest` instance.
    ///
    /// This method takes the current configuration stored in `http_request`, creates a new
    /// `HttpRequest` instance with the configuration, and resets the builder's temporary
    /// state for further use.
    ///
    /// # Returns
    /// Returns a fully constructed `HttpRequest` instance based on the current builder state.
    pub fn builder(&mut self) -> HttpRequest {
        self.builder = self.http_request.clone();
        self.http_request = HttpRequest::default();
        self.builder.clone()
    }
}
