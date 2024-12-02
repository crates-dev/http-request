## http-request

[![](https://img.shields.io/crates/v/http-request.svg)](https://crates.io/crates/http-request)
[![](https://docs.rs/http-request/badge.svg)](https://docs.rs/http-request)
[![](https://img.shields.io/crates/l/http-request.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/http-request/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/http-request/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/HTTP-REQUEST/)

> http-request is a lightweight, efficient library for building, sending, and handling HTTP/HTTPS requests in Rust applications. It provides a simple and intuitive API, allowing developers to easily interact with web services, whether they use the "HTTP" or "HTTPS" protocol. The library supports various HTTP methods, custom headers, request bodies, timeout, and automatic handling of redirects (including detecting redirect loops), enabling fast and secure communication. Whether working with secure "HTTPS" connections or standard "HTTP" requests, the library is optimized for performance, minimal resource usage, and easy integration into Rust projects.

## Features

- **Support for HTTP/HTTPS**: Supports both HTTP and HTTPS protocols.
- **Lightweight Design**: The `http_request` crate provides a simple and efficient API for building, sending, and handling HTTP requests while minimizing resource consumption.
- **Supports Common HTTP Methods**: Supports common HTTP methods such as GET and POST.
- **Flexible Request Building**: Offers rich configuration options through `HttpRequestBuilder` to set request headers, bodies, and URLs.
- **Simple Error Handling**: Utilizes the `Result` type to handle errors in requests and responses, making error handling straightforward.
- **Custom Headers and Request Bodies**: Easily add custom headers and request bodies.
- **Response Handling**: Provides a simple wrapper around HTTP responses, making it easy to access and process response data.
- **Optimized Memory Management**: Implements efficient memory management to minimize unnecessary memory allocations and improve performance.
- **Redirect Handling**: Supports redirect handling, allows setting the maximum number of redirects, and includes redirect loop detection.
- **timeout**: Supports timeout

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
let mut header: HashMap<&str, &str> = HashMap::new();
header.insert("header-key", "header-value");
let mut _request_builder = HttpRequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .builder();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

### Send post request

#### Send Body Json

```rs
use http_request::*;
use std::collections::HashMap;
let mut header: HashMap<&str, &str> = HashMap::new();
header.insert("header-key", "header-value");
let mut body: HashMap<&str, &str> = HashMap::new();
body.insert("body-key", "body-value");
let mut _request_builder = HttpRequestBuilder::new()
    .post("http://localhost:80")
    .json(body)
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .builder();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

#### Send Body Text

```rs
use http_request::*;
use std::collections::HashMap;
let mut header: HashMap<&str, &str> = HashMap::new();
header.insert("header-key", "header-value");
let mut _request_builder = HttpRequestBuilder::new()
    .post("http://localhost")
    .text("hello")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .builder();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

#### Send Body Binary

```rs
use http_request::*;
use std::collections::HashMap;
let mut header: HashMap<&str, &str> = HashMap::new();
header.insert("header-key", "header-value");
let mut _request_builder = HttpRequestBuilder::new()
    .post("http://localhost")
    .body("hello".as_bytes())
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .builder();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
