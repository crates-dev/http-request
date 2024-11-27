use std::collections::HashMap;

/// 表示HTTP响应的结构体
pub struct HttpResponse {
    // HTTP版本
    pub http_version: String,
    // HTTP状态码
    pub status_code: u16,
    // 状态文本 (例如 "OK" 或 "Not Found")
    pub status_text: String,
    // 响应头，键值对形式
    pub headers: HashMap<String, String>,
    // 响应体
    pub body: String,
}

/// HTTP状态码枚举
pub enum HttpStatusCode {
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
}
