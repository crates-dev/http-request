/// A trait representing common behaviors for HTTP response types.
///
/// This trait provides a generic `text` method and a `from` method for
/// parsing and transforming HTTP responses.
///
/// # Associated Types
/// - `Output`: The type returned by the `from` method.
pub trait HttpResponse {
    type OutputText;
    type OutputBinary;

    /// Transforms the HTTP response into a text representation.
    ///
    /// # Returns
    /// Returns the body of the HTTP response as a string.
    fn text(self) -> Self::OutputText;

    /// Parses a raw HTTP response into the associated type `Output`.
    ///
    /// # Parameters
    /// - `response`: A byte slice representing the raw HTTP response.
    ///
    /// # Returns
    /// Returns an instance of the implementing type.
    fn from(response: &[u8]) -> Self::OutputBinary;
}
