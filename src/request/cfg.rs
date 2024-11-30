use crate::*;
use color_output::*;
use std_macro_extensions::*;

#[allow(dead_code)]
fn output(title: &str, msg: &str, color: Color) {
    OutputBuilder::new()
        .set_show_time(true)
        .set_time_bg_color(ColorType::Use(Color::Cyan))
        .set_time_text_blod(true)
        .set_split_text(title)
        .set_split_text_blod(true)
        .set_split_bg_color(ColorType::Use(Color::Yellow))
        .set_text(msg)
        .set_text_bg_color(ColorType::Use(color))
        .set_text_blod(true)
        .set_endl(true)
        .build()
        .output();
}

#[test]
fn test_post_request() {
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("header-key".to_string(), "header-value".to_string());
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("body-key".to_string(), "body-value".to_string());
    let mut _request_builder = HttpRequestBuilder::new()
        .set_methods(Methods::POST)
        .set_url("http://localhost:80")
        .set_body(&body)
        .set_header(&header)
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
fn test_get_request() {
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("header-key".to_string(), "header-value".to_string());
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("body-key".to_string(), "body-value".to_string());
    let mut _request_builder = HttpRequestBuilder::new()
        .set_methods(Methods::GET)
        .set_url("http://localhost:80")
        .set_header(&header)
        .builder();
    _request_builder
        .send()
        .and_then(|response| {
            output("response => ", &format!("{:?}", response), Color::Green);
            Ok(())
        })
        .unwrap_or_else(|e| output("error => ", &format!("{:?}", e), Color::Red));
}
