use std::collections::HashMap;

#[derive(Debug, Clone)]
/// 表示HTTP响应的结构体
pub struct HttpResponse {
    // HTTP版本
    pub http_version: String,
    // HTTP状态码
    pub status_code: u16,
    // 状态文本
    pub status_text: String,
    // 响应头
    pub headers: HashMap<String, String>,
    // 响应体
    pub body: String,
}
