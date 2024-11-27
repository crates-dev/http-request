use super::r#type::HttpRequest;
use crate::{
    global_type::r#type::{Body, Header},
    Methods, Protocol,
};
use crate::{
    request::{
        constant::{HTTP_BR, HTTP_DOUBLE_BR},
        error::Error,
    },
    request_url::r#type::Url,
};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

impl HttpRequest {
    fn get_protocol(&self) -> Protocol {
        self.protocol.as_ref().clone()
    }

    fn get_methods(&self) -> Methods {
        self.methods.as_ref().clone()
    }

    fn get_url(&self) -> String {
        self.url.as_ref().clone()
    }

    fn get_header(&self) -> Header {
        self.header.as_ref().clone()
    }

    fn get_body(&self) -> Body {
        self.body.as_ref().clone()
    }

    fn set_url(&mut self, url: String) {
        self.url = Arc::new(url);
    }

    fn parse_url(&self) -> Result<Url, Error> {
        if let Ok(parse_res) = Url::parse(&self.get_url()) {
            Ok(parse_res)
        } else {
            Err(Error::InvalidUrl)
        }
    }

    fn get_header_str(&self) -> String {
        let header: HashMap<String, String> = self.get_header();
        let mut header_string: String = String::new();
        for (key, value) in header {
            let line: String = format!("{}: {}{}", key, value, HTTP_BR);
            header_string.push_str(&line);
        }
        header_string
    }

    fn get_body_str(&self) -> String {
        let body: HashMap<String, String> = self.get_body();
        let body_str: String = body.iter().map(|(k, v)| format!("{}={}&", k, v)).collect();
        let body_str: &str = body_str.trim_end_matches('&');
        body_str.to_owned()
    }

    fn send_get_request(&mut self, stream: &mut TcpStream, url_obj: &Url) -> String {
        let path: String = url_obj.path.clone().unwrap_or("/".to_string());
        let mut request: String = format!("GET {} HTTP/1.1{}", path, HTTP_BR);
        request.push_str(&format!(
            "Host: {}{}",
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        request.push_str(HTTP_BR);
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    fn send_post_request(&mut self, stream: &mut TcpStream, url_obj: &Url) -> String {
        let path: String = url_obj.path.clone().unwrap_or("/".to_string());
        let mut request: String = format!("POST {} HTTP/1.1{}", path, HTTP_BR);
        request.push_str(&format!(
            "Host: {}{}",
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        let body_str: String = self.get_body_str();
        request.push_str(&format!("Content-Length: {}{}", body_str.len(), HTTP_BR));
        request.push_str(HTTP_BR);
        request.push_str(&format!("{}{}", body_str, HTTP_BR));
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    fn read_response(&mut self, stream: &mut TcpStream) -> String {
        let mut buffer: [u8; 10240] = [0; 10240];
        let mut response: String = String::new();
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<String> = None;

        while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }
            response.push_str(&String::from_utf8_lossy(&buffer[..n]));
            if !headers_done {
                if let Some(pos) = response.find(HTTP_DOUBLE_BR) {
                    headers_done = true;

                    // 检查是否有重定向
                    if let Some(status_pos) = response.find("HTTP/1.1") {
                        let status_code = response[status_pos + 9..status_pos + 12]
                            .trim()
                            .parse::<usize>()
                            .unwrap_or(0);
                        if (300..=399).contains(&status_code) {
                            // 找到 Location 头部
                            if let Some(location_pos) = response.to_lowercase().find("location:") {
                                let start = location_pos + "location:".len();
                                if let Some(end) = response[start..].find(HTTP_BR) {
                                    redirect_url =
                                        Some(response[start..start + end].trim().to_string());
                                    break;
                                }
                            }
                        }
                    }

                    // 获取 Content-Length
                    if let Some(length_pos) = response.to_lowercase().find("content-length:") {
                        let start = length_pos + "content-length:".len();
                        if let Some(end) = response[start..].find(HTTP_BR) {
                            content_length =
                                response[start..start + end].trim().parse().unwrap_or(0);
                        }
                    }
                    response = response.split_off(pos + 4);
                }
            }

            // 读取完所有内容
            if headers_done && response.len() >= content_length {
                break;
            }
        }

        // 如果有重定向，则返回重定向结果
        if let Some(url) = redirect_url {
            // 这里可以进行重定向处理，例如发起新请求
            println!("Redirecting to: {}", url);
            // 可以调用一个处理重定向的方法，比如再请求一次新地址
            if let Ok(_res) = self.handle_redirect(url) {
                _res
            } else {
                String::new()
            }
        } else {
            response
        }
    }

    fn handle_redirect(&mut self, url: String) -> Result<String, Error> {
        // 使用 url crate 解析 URL
        self.set_url(url.clone());
        if let Ok(url_obj) = self.parse_url() {
            let host: String = url_obj.host.unwrap_or_default();
            let port: u16 = self.get_port(url_obj.port.clone().unwrap_or_default());
            let path: String = url_obj.path.unwrap_or_default();
            // 生成新的 HTTP 请求
            let request = format!(
                "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                path, host
            );
            println!("{}", request);

            // 建立到新主机和端口的连接
            let address: String = format!("{}:{}", host, port);
            let mut stream: TcpStream =
                TcpStream::connect(&address).expect("Failed to connect to the host");
            stream.write_all(request.as_bytes()).unwrap();

            // 递归调用读取新的响应
            Ok(self.read_response(&mut stream))
        } else {
            Err(Error::InvalidUrl)
        }
    }

    fn get_port(&self, port: u16) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol: Protocol = self.get_protocol();
        protocol.get_port()
    }

    pub fn send(&mut self) -> Result<String, Error> {
        if let Ok(url_obj) = self.parse_url() {
            let methods: Methods = self.get_methods();
            let host: String = url_obj.host.clone().unwrap_or_default();
            let port: u16 = self.get_port(url_obj.port.clone().unwrap_or_default());
            TcpStream::connect((host, port))
                .map_err(|_| Error::TcpStreamConnectError) // 转换 TcpStream 的错误
                .and_then(|mut stream| match methods {
                    m if m.is_get() => Ok(self.send_get_request(&mut stream, &url_obj)),
                    m if m.is_post() => Ok(self.send_post_request(&mut stream, &url_obj)),
                    _ => Err(Error::RequestError),
                })
        } else {
            Err(Error::InvalidUrl)
        }
    }
}

impl Default for HttpRequest {
    fn default() -> HttpRequest {
        HttpRequest {
            methods: Arc::new(Methods::new()),
            url: Arc::new(String::new()),
            protocol: Arc::new(Protocol::new()),
            header: Arc::new(HashMap::new()),
            body: Arc::new(HashMap::new()),
        }
    }
}
