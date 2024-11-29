## http request

> Http request is a lightweight, efficient library for building, sending, and handling HTTP requests in Rust applications.

## Use

### Send post request

```rs
use http_request::*;
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
        println!("{:?}", response);
        Ok(())
    })
    .unwrap_or_else(|e| println!("error => {}", e));
```

### Send get request

```rs
use http_request::*;
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
        println!("{:?}", response);
        Ok(())
    })
    .unwrap_or_else(|e| println!("error => {}", e));
```
