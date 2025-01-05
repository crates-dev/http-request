/// Trait representing an HTTP request.
///
/// Provides a method for sending an HTTP request and obtaining the result.
pub trait RequestTrait: Send {
    /// Associated type for the result of the HTTP request.
    type RequestResult;

    /// Sends the HTTP request.
    ///
    /// - Returns: The associated type `RequestResult` which represents the outcome of the HTTP request.
    fn send(&mut self) -> Self::RequestResult;
}
