/// Enumeration of HTTP status codes representing various HTTP response statuses
///
/// This enum includes common HTTP status codes that cover successful requests, client errors,
/// and server errors. Each variant represents a specific HTTP status code.
///
/// # Variants
/// - `Ok`: HTTP 200, indicates that the request was successful and the server has successfully processed it.
/// - `Created`: HTTP 201, indicates that the request was successful and the server has created a new resource.
/// - `NoContent`: HTTP 204, indicates that the request was successful but no content is returned.
/// - `BadRequest`: HTTP 400, indicates that the request is invalid or malformed and the server cannot understand it.
/// - `Unauthorized`: HTTP 401, indicates that the request is unauthorized and requires authentication.
/// - `Forbidden`: HTTP 403, indicates that the server understands the request but refuses to authorize it, usually due to insufficient permissions.
/// - `NotFound`: HTTP 404, indicates that the server cannot find the requested resource.
/// - `InternalServerError`: HTTP 500, indicates that the server encountered an internal error and cannot process the request.
/// - `NotImplemented`: HTTP 501, indicates that the server does not support the functionality required to fulfill the request.
/// - `BadGateway`: HTTP 502, indicates that the server, while acting as a gateway or proxy, received an invalid response from the upstream server.
/// - `Unknown`: Represents an unknown status code, typically used when the status code is not recognized or is undefined.
/// ```
pub enum StatusCode {
    /// 200 OK
    Ok,
    /// 201 Created
    Created,
    /// 204 No Content
    NoContent,
    /// 400 Bad Request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 403 Forbidden
    Forbidden,
    /// 404 Not Found
    NotFound,
    /// 500 Internal Server Error
    InternalServerError,
    /// 501 Not Implemented
    NotImplemented,
    /// 502 Bad Gateway
    BadGateway,
    /// Unknown status code
    Unknown,
}
