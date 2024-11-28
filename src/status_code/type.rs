#[allow(dead_code)]
/// HTTP状态码枚举
pub enum StatusCode {
    // 200 OK
    Ok,
    // 201 Created
    Created,
    // 204 No Content
    NoContent,
    // 400 Bad Request
    BadRequest,
    // 401 Unauthorized
    Unauthorized,
    // 403 Forbidden
    Forbidden,
    // 404 Not Found
    NotFound,
    // 500 Internal Server Error
    InternalServerError,
    // 501 Not Implemented
    NotImplemented,
    // 502 Bad Gateway
    BadGateway,
    // Unknown
    Unknown,
}
