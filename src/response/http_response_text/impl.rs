use super::r#type::HttpResponseText;
use crate::response::{http_response_binary::r#type::HttpResponseBinary, r#trait::HttpResponse};

/// Implements the `HttpResponse` trait for `HttpResponseText`.
///
/// This implementation allows `HttpResponseText` to convert between text and binary
/// representations of HTTP responses. It provides methods for parsing raw responses, as well
/// as accessing text and binary formats.
///
/// # Associated Types
/// - `OutputText`: Specifies the text representation of an HTTP response (`HttpResponseText`).
/// - `OutputBinary`: Specifies the binary representation of an HTTP response (`HttpResponseBinary`).
impl HttpResponse for HttpResponseText {
    type OutputText = HttpResponseText;
    type OutputBinary = HttpResponseBinary;

    /// Parses a raw HTTP response from a byte slice and converts it to a `HttpResponseText` instance.
    ///
    /// This method utilizes the `from` implementation of `HttpResponseBinary` to parse the binary
    /// response and then converts it to a text representation.
    ///
    /// # Parameters
    /// - `response`: A byte slice representing the raw HTTP response.
    ///
    /// # Returns
    /// - `Self::OutputText`: A `HttpResponseText` instance with the parsed response.
    ///
    /// # Panics
    /// - This method will panic if the binary parsing or text conversion fails unexpectedly.
    fn from(response: &[u8]) -> Self::OutputText
    where
        Self: Sized,
    {
        <HttpResponseBinary as HttpResponse>::from(response).text()
    }

    /// Returns a clone of the current text representation of the HTTP response.
    ///
    /// This method allows for retrieving the current instance as the text representation without
    /// modification.
    ///
    /// # Returns
    /// - `Self::OutputText`: A clone of the current instance.
    fn text(&self) -> Self::OutputText {
        self.clone()
    }

    /// Converts the text representation to a binary representation of the HTTP response.
    ///
    /// This method constructs a new `HttpResponseBinary` instance, copying all fields and
    /// converting the body from a string to a byte vector.
    ///
    /// # Returns
    /// - `HttpResponseBinary`: The binary representation of the HTTP response.
    fn binary(&self) -> HttpResponseBinary {
        HttpResponseBinary {
            http_version: self.http_version.clone(),
            status_code: self.status_code,
            status_text: self.status_text.clone(),
            headers: self.headers.clone(),
            body: self.body.clone().into_bytes(),
        }
    }
}
