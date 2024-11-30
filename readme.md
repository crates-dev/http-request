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

### Send get request

```rs
use http_request::*;
use std::collections::HashMap;
let mut header: HashMap<String, String> = HashMap::new();
header.insert("header-key"., "header-value".);
let mut _request_builder = HttpRequestBuilder::new()
    .get("http://localhost:80")
    .headers(header)
    .timeout(6000)
    .builder();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response);
        Ok(())
    })
    .unwrap_or_else(|e| println!("error => {}", e));
```

### Send post request

#### Send Body Json

```rs
use http_request::*;
use std::collections::HashMap;
let mut header: HashMap<String, String> = HashMap::new();
header.insert("header-key"., "header-value".);
let mut body: HashMap<String, String> = HashMap::new();
body.insert("body-key"., "body-value".);
let mut _request_builder = HttpRequestBuilder::new()
    .post("http://localhost:80")
    .json(body)
    .headers(header)
    .timeout(6000)
    .builder();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response);
        Ok(())
    })
    .unwrap_or_else(|e| println!("error => {}", e));
```

#### Send Body Text

```rs
use http_request::*;
use std::collections::HashMap;
let mut header: HashMap<String, String> = HashMap::new();
header.insert("header-key"., "header-value".);
let mut _request_builder = HttpRequestBuilder::new()
    .post("http://localhost:80")
    .text("hello")
    .headers(header)
    .timeout(6000)
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
