## http request

> Http request is a lightweight, efficient library for building, sending, and handling HTTP requests in Rust applications.

## Features

- **Lightweight Design**: The http_request crate provides a simple and efficient API for building, sending, and handling HTTP requests, designed to minimize resource consumption.
- **Supports Common HTTP Methods**: Supports various HTTP methods such as GET, POST.
  Flexible Request Building: Offers rich configuration options through HttpRequestBuilder to set request headers, body, and URL.
- **Simple Error Handling**: Utilizes the Result type for handling errors in requests and responses, making it easy to deal with error scenarios.
- **Custom Headers and Body**: Easily add custom headers and request bodies to the HTTP request.
- **Response Handling**: Provides a simple wrapper around HTTP responses, allowing you to easily access and process the response data.
- **Optimized Memory Management**: Implements efficient memory handling, minimizing unnecessary memory allocations and improving performance.

## Installation

To use this crate, you can run cmd:

```shell
cargo add http-request
```

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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
