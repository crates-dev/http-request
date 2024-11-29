use super::r#type::HttpRequest;
use crate::{
    constant::http::{
        CONNECTION, CONTENT_LENGTH, DEFAULT_HTTP_PATH, DEFAULT_HTTP_VERSION, HOST, LOCATION,
    },
    global_type::r#type::{Body, Header},
    response::r#type::HttpResponse,
    Methods, Protocol, *,
};
use crate::{
    constant::http::{HTTP_BR, HTTP_DOUBLE_BR},
    request::error::Error,
    request_url::r#type::RequestUrl,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
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

    fn parse_url(&self) -> Result<RequestUrl, Error> {
        if let Ok(parse_res) = RequestUrl::parse(&self.get_url()) {
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
        let body_string: String = body
            .iter()
            .filter(|(key, value)| !key.is_empty() && !value.is_empty())
            .map(|(key, value)| format!("{}={}&", key, value))
            .collect::<Vec<String>>()
            .join("&");
        body_string
    }

    fn send_get_request(&mut self, stream: &mut TcpStream, url_obj: &RequestUrl) -> HttpResponse {
        let path: String = url_obj.path.clone().unwrap_or("/".to_string());
        let mut request: String = format!(
            "{} {} {}{}",
            Methods::GET,
            path,
            DEFAULT_HTTP_VERSION,
            HTTP_BR
        );
        request.push_str(&format!(
            "{}: {}{}",
            HOST,
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        request.push_str(HTTP_BR);
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    fn send_post_request(&mut self, stream: &mut TcpStream, url_obj: &RequestUrl) -> HttpResponse {
        let path: String = url_obj
            .path
            .clone()
            .unwrap_or(DEFAULT_HTTP_PATH.to_string());
        let mut request: String = format!(
            "{} {} {}{}",
            path,
            Methods::POST,
            DEFAULT_HTTP_VERSION,
            HTTP_BR
        );
        request.push_str(&format!(
            "{}: {}{}",
            HOST,
            url_obj.host.as_ref().unwrap_or(&"".to_string()),
            HTTP_BR
        ));
        request.push_str(&self.get_header_str());
        let body_str: String = self.get_body_str();
        request.push_str(&format!(
            "{}: {}{}",
            CONTENT_LENGTH,
            body_str.len(),
            HTTP_BR
        ));
        request.push_str(HTTP_BR);
        request.push_str(&format!("{}{}", body_str, HTTP_BR));
        stream.write_all(request.as_bytes()).unwrap();
        self.read_response(stream)
    }

    fn read_response(&mut self, stream: &mut TcpStream) -> HttpResponse {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut response_string: String = String::new();
        let mut headers_done: bool = false;
        let mut content_length: usize = 0;
        let mut redirect_url: Option<String> = None;

        'first_loop: while let Ok(n) = stream.read(&mut buffer) {
            if n == 0 {
                break;
            }
            response_string.push_str(&String::from_utf8_lossy(&buffer[..n]));
            if !headers_done {
                if response_string.contains(HTTP_DOUBLE_BR) {
                    headers_done = true;
                    response_string
                        .find(DEFAULT_HTTP_VERSION)
                        .and_then(|status_pos| {
                            let status_code = response_string[status_pos + 9..status_pos + 12]
                                .trim()
                                .parse::<usize>()
                                .unwrap_or_default();
                            if (300..=399).contains(&status_code) {
                                let location_sign_key: String =
                                    format!("{}:", LOCATION.to_lowercase());
                                response_string
                                    .to_lowercase()
                                    .find(&location_sign_key)
                                    .and_then(|location_pos| {
                                        let start: usize = location_pos + location_sign_key.len();
                                        response_string[start..].find(HTTP_BR).and_then(|end| {
                                            redirect_url = Some(
                                                response_string[start..start + end]
                                                    .trim()
                                                    .to_string(),
                                            );
                                            return Some(());
                                        });
                                        return Some(());
                                    });
                            }
                            return Some(());
                        });
                    content_length = HttpResponse::get_content_length(&response_string);
                }
            }
            if headers_done && response_string.len() >= content_length {
                break 'first_loop;
            }
        }
        let response: HttpResponse = HttpResponse::from(&response_string);
        let res: HttpResponse = redirect_url
            .and_then(|url| {
                // 重定向处理
                let redirect_res: HttpResponse = if let Ok(_res) = self.handle_redirect(url) {
                    _res
                } else {
                    response.clone()
                };
                Some(redirect_res)
            })
            .unwrap_or_else(|| response);
        res
    }

    fn handle_redirect(&mut self, url: String) -> Result<HttpResponse, Error> {
        self.set_url(url.clone());
        self.parse_url()
            .map_err(|_| Error::InvalidUrl)
            .and_then(|url_obj| {
                let host: String = url_obj.host.unwrap_or_default();
                let port: u16 = self.get_port(url_obj.port.clone().unwrap_or_default());
                let path: String = url_obj.path.unwrap_or_default();
                let request: String = format!(
                    "{} {} {}{}{}: {}{}{}: close{}",
                    Methods::GET,
                    path,
                    DEFAULT_HTTP_VERSION,
                    HTTP_DOUBLE_BR,
                    HOST,
                    host,
                    HTTP_DOUBLE_BR,
                    CONNECTION,
                    HTTP_DOUBLE_BR
                );
                let address: String = format!("{}:{}", host, port);
                TcpStream::connect(&address)
                    .map_err(|_| Error::TcpStreamConnectError)
                    .and_then(|mut stream| {
                        stream.write_all(request.as_bytes()).unwrap();
                        Ok(self.read_response(&mut stream))
                    })
            })
    }

    fn get_port(&self, port: u16) -> u16 {
        if port != 0 {
            return port;
        }
        let protocol: Protocol = self.get_protocol();
        protocol.get_port()
    }

    pub fn send(&mut self) -> Result<HttpResponse, Error> {
        self.parse_url()
            .map_err(|_| Error::InvalidUrl)
            .and_then(|url_obj| {
                let methods: Methods = self.get_methods();
                let host: String = url_obj.host.clone().unwrap_or_default();
                let port: u16 = self.get_port(url_obj.port.clone().unwrap_or_default());
                TcpStream::connect((host, port))
                    .map_err(|_| Error::TcpStreamConnectError)
                    .and_then(|mut stream| {
                        let res: Result<HttpResponse, Error> = match methods {
                            m if m.is_get() => Ok(self.send_get_request(&mut stream, &url_obj)),
                            m if m.is_post() => Ok(self.send_post_request(&mut stream, &url_obj)),
                            _ => Err(Error::RequestError),
                        };
                        res
                    })
            })
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
