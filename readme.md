<center>

## http-request

[![](https://img.shields.io/crates/v/http-request.svg)](https://crates.io/crates/http-request)
[![](https://img.shields.io/crates/d/http-request.svg)](https://img.shields.io/crates/d/http-request.svg)
[![](https://docs.rs/http-request/badge.svg)](https://docs.rs/http-request)
[![](https://github.com/ltpp-universe/http-request/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/http-request/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/http-request.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/HTTP-REQUEST/)

[Api Docs](https://docs.rs/http-request/latest/http_request/)

> http-request is a lightweight, efficient library for building, sending, and handling HTTP/HTTPS requests in Rust applications. It provides a simple and intuitive API, allowing developers to easily interact with web services, whether they use the "HTTP" or "HTTPS" protocol. The library supports various HTTP methods, custom headers, request bodies, timeout, automatic handling of redirects (including detecting redirect loops), and enhanced response body decoding (both automatic and manual), enabling fast and secure communication. Whether working with secure "HTTPS" connections or standard "HTTP" requests, the library is optimized for performance, minimal resource usage, and easy integration into Rust projects.http-request is a lightweight, efficient library for building, sending, and handling HTTP/HTTPS requests in Rust applications. It provides a simple and intuitive API, allowing developers to easily interact with web services, whether they use the "HTTP" or "HTTPS" protocol. The library supports various HTTP methods, custom headers, request bodies, timeout, automatic handling of redirects (including detecting redirect loops), and enhanced response body decoding (both automatic and manual), enabling fast and secure communication. Whether working with secure "HTTPS" connections or standard "HTTP" requests, the library is optimized for performance, minimal resource usage, and easy integration into Rust projects.

## Features

- **Support for HTTP/HTTPS**: Supports both HTTP and HTTPS protocols.
- **Lightweight Design**: The `http_request` crate provides a simple and efficient API for building, sending, and handling HTTP requests while minimizing resource consumption.
- **Supports Common HTTP Methods**: Supports common HTTP methods such as GET and POST.
- **Flexible Request Building**: Offers rich configuration options through `RequestBuilder` to set request headers, bodies, and URLs.
- **Simple Error Handling**: Utilizes the `Result` type to handle errors in requests and responses, making error handling straightforward.
- **Custom Headers and Request Bodies**: Easily add custom headers and request bodies.
- **Response Handling**: Provides a simple wrapper around HTTP responses, making it easy to access and process response data.
- **Optimized Memory Management**: Implements efficient memory management to minimize unnecessary memory allocations and improve performance.
- **Redirect Handling**: Supports redirect handling, allows setting the maximum number of redirects, and includes redirect loop detection.
- **timeout**: Supports timeout.
- **Automatic and Manual Response Body Decoding**: Supports both automatic and manual decoding of response bodies, allowing for seamless interaction with different content types (e.g., JSON, XML, etc.).

## Installation

To use this crate, you can run cmd:

```shell
cargo add http-request
```

## Use

### Send get request

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut _request_builder = RequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .build();
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

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut body: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
body.insert("body-key", "body-value");
let mut _request_builder = RequestBuilder::new()
    .post("http://localhost:80")
    .json(body)
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .build();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.decode().text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

#### Send Body Text

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut _request_builder = RequestBuilder::new()
    .post("http://localhost")
    .text("hello")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .build();
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

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut _request_builder = RequestBuilder::new()
    .post("http://localhost")
    .body("hello".as_bytes())
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .build();
_request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.decode(4096).text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

## Help

Ensure that CMake is installed on the system

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
