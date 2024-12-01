use crate::*;
use color_output::*;
use std_macro_extensions::*;

#[allow(dead_code)]
fn output(title: &str, msg: &str, color: Color) {
    OutputBuilder::new()
        .show_time(true)
        .time_bg_color(ColorType::Use(Color::Cyan))
        .time_text_blod(true)
        .split_text(title)
        .split_text_blod(true)
        .split_bg_color(ColorType::Use(Color::Yellow))
        .text(msg)
        .text_bg_color(ColorType::Use(color))
        .text_blod(true)
        .endl(true)
        .build()
        .output();
}

#[test]
fn test_http_post_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    header.insert("Connection", "keep-alive");
    header.insert("Accept-Encoding", "gzip, deflate");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("code", "hello");
    body.insert("language", "rust");
    body.insert("testin", "");
    let mut _request_builder = HttpRequestBuilder::new()
        .post("http://localhost:80/rust?hello=rust")
        .json(body)
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "response => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_http_get_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("header-key", "header-value");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("body-key", "body-value");
    let mut _request_builder = HttpRequestBuilder::new()
        .get("http://localhost:80")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "response => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_https_post_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    header.insert("Connection", "keep-alive");
    header.insert("Accept-Encoding", "gzip, deflate");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("code", "fn main() {\r\n    println!(\"hello world\");\r\n}");
    body.insert("language", "rust");
    body.insert("testin", "");
    let mut _request_builder = HttpRequestBuilder::new()
        .post("https://code.ltpp.vip/")
        .json(body)
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "response => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_https_get_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("header-key", "header-value");
    let mut body: HashMap<&str, &str> = HashMap::new();
    body.insert("body-key", "body-value");
    let mut _request_builder = HttpRequestBuilder::new()
        .get("https://code.ltpp.vip/")
        .headers(header)
        .timeout(10000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "response => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_http_post_text_request() {
    let mut header: HashMap<&str, &str> = HashMap::new();
    header.insert("Accept", "*/*");
    header.insert("Content-Type", "application/json");
    let mut _request_builder = HttpRequestBuilder::new()
        .post("http://localhost:80")
        .text("hello")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output(
                "response => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}
