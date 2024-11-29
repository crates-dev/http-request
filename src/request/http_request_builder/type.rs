use crate::request::http_request::r#type::HttpRequest;

/// Builder pattern for constructing `HttpRequest` instances.
///
/// The `HttpRequestBuilder` struct facilitates the creation of `HttpRequest` objects
/// through a series of method calls. It allows for flexible and clear configuration of
/// an HTTP request's components such as method, URL, headers, and body.
///
/// # Fields
/// - `tmp`: A temporary `HttpRequest` instance used to accumulate changes during
///   the construction process. It holds the current state of the builder.
/// - `builder`: A finalized `HttpRequest` instance that holds the result after the
///   builder process has been completed. It is returned when the builder is finalized.
///
/// # Traits Implemented
/// - `Debug`: Enables the ability to format and print the `HttpRequestBuilder` for debugging purposes.
/// - `Clone`: Allows for cloning of the `HttpRequestBuilder` and its internal components.
/// - `PartialEq`: Provides equality comparison between two `HttpRequestBuilder` instances.
///
/// This builder simplifies the creation of `HttpRequest` objects, ensuring thread-safety
/// and immutability of shared references, while providing a fluent API for constructing
/// HTTP requests with various configurations.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequestBuilder {
    pub(crate) tmp: HttpRequest,
    pub(crate) builder: HttpRequest,
}
