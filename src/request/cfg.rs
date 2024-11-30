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
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("header-key".to_string(), "header-value".to_string());
    header.insert(":authority".to_string(), "code.ltpp.vip".to_string());
    header.insert(":method".to_string(), "POST".to_string());
    header.insert(":path".to_string(), "/".to_string());
    header.insert(":scheme".to_string(), "https".to_string());
    header.insert("Accept".to_string(), "*/*".to_string());
    header.insert("Content-Type".to_string(), "application/json".to_string());
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("body-key".to_string(), "body-value".to_string());
    let mut _request_builder = HttpRequestBuilder::new()
        .post("http://localhost:80")
        .body(&body)
        .headers(&header)
        .timeout(6000)
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output("response => ", &format!("{:?}", response), Color::Green);
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_http_get_request() {
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("header-key".to_string(), "header-value".to_string());
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("body-key".to_string(), "body-value".to_string());
    let mut _request_builder = HttpRequestBuilder::new()
        .get("http://localhost:80")
        .headers(&header)
        .timeout(6000)
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output("response => ", &format!("{:?}", response), Color::Green);
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_https_post_request() {
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert(":authority".to_string(), "code.ltpp.vip".to_string());
    header.insert(":method".to_string(), "POST".to_string());
    header.insert(":path".to_string(), "/".to_string());
    header.insert(":scheme".to_string(), "https".to_string());
    header.insert("Accept".to_string(), "*/*".to_string());
    header.insert("Content-Type".to_string(), "application/json".to_string());
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert(
        "code".to_string(),
        "fn main() {\r\n    println!(\"hello world\");\r\n}".to_string(),
    );
    body.insert("language".to_string(), "rust".to_string());
    body.insert("testin".to_string(), "".to_string());
    let mut _request_builder = HttpRequestBuilder::new()
        .post("https://code.ltpp.vip/")
        .body(&body)
        .headers(&header)
        .timeout(6000)
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output("response => ", &format!("{:?}", response), Color::Green);
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_https_get_request() {
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("header-key".to_string(), "header-value".to_string());
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("body-key".to_string(), "body-value".to_string());
    let mut _request_builder = HttpRequestBuilder::new()
        .get("https://git.ltpp.vip/root")
        .headers(&header)
        .timeout(6000)
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output("response => ", &format!("{:?}", response), Color::Green);
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}
