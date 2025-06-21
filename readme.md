<center>

## http-request

[![](https://img.shields.io/crates/v/http-request.svg)](https://crates.io/crates/http-request)
[![](https://img.shields.io/crates/d/http-request.svg)](https://img.shields.io/crates/d/http-request.svg)
[![](https://docs.rs/http-request/badge.svg)](https://docs.rs/http-request)
[![](https://github.com/eastspire/http-request/workflows/Rust/badge.svg)](https://github.com/eastspire/http-request/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/http-request.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/HTTP-REQUEST/)

[Api Docs](https://docs.rs/http-request/latest/http_request/)

> http-request is a lightweight, efficient library for building, sending, and handling HTTP/HTTPS requests in Rust applications. It provides a simple and intuitive API, allowing developers to easily interact with web services, whether they use the "HTTP" or "HTTPS" protocol. The library supports various HTTP methods, custom headers, request bodies, timeout, automatic handling of redirects (including detecting redirect loops), and enhanced response body decoding (both automatic and manual), enabling fast and secure communication. Whether working with secure "HTTPS" connections or standard "HTTP" requests, the library is optimized for performance, minimal resource usage, and easy integration into Rust projects.http-request is a lightweight, efficient library for building, sending, and handling HTTP/HTTPS requests in Rust applications. It provides a simple and intuitive API, allowing developers to easily interact with web services, whether they use the "HTTP" or "HTTPS" protocol. The library supports various HTTP methods, custom headers, request bodies, timeout, automatic handling of redirects (including detecting redirect loops), and enhanced response body decoding (both automatic and manual), enabling fast and secure communication. Whether working with secure "HTTPS" connections or standard "HTTP" requests, the library is optimized for performance, minimal resource usage, and easy integration into Rust projects.

## Features

- **Support for HTTP/HTTPS**: Supports both HTTP and HTTPS protocols.
- **Lightweight Design**: The `http_request` crate provides a simple and efficient API for building, sending, and handling HTTP requests while minimizing resource consumption.
- **Supports Common HTTP Method**: Supports common HTTP methods such as GET and POST.
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

## Synchronous

### Send get request

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .build_sync();
request_builder
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
let body: JsonValue = json_value!({
    "test": 1
});
let mut request_builder = RequestBuilder::new()
    .post("http://code.ltpp.vip")
    .json(body)
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .build_sync();
request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.decode(4096).text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

#### Send Body Text

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .post("http://ide.ltpp.vip/?language=rust")
    .text("hello")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .build_sync();
request_builder
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
let mut request_builder = RequestBuilder::new()
    .post("http://ide.ltpp.vip/?language=rust")
    .body("hello".as_bytes())
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .build_sync();
request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.decode(4096).text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

### Send request with HTTP proxy

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .http_proxy("127.0.0.1", 7890)
    .build_sync();
request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

### Send request with HTTP proxy authentication

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .http_proxy_auth("127.0.0.1", 7890, "username", "password")
    .build_sync();
request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

### Send request with SOCKS5 proxy

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("http://ide.ltpp.vip/?language=rust")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .socks5_proxy("127.0.0.1", 1080)
    .build_sync();
request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

### Send request with SOCKS5 proxy authentication

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("http://ide.ltpp.vip/?language=rust")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .socks5_proxy_auth("127.0.0.1", 1080, "username", "password")
    .build_sync();
request_builder
    .send()
    .and_then(|response| {
        println!("{:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {}", e));
```

## Asynchronous

### Send get request

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.text());
    }
    Err(e) => println!("Error => {}", e),
}
```

### Send post request

#### Send Body Json

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let body: JsonValue = json_value!({
    "test": 1
});
let mut request_builder = RequestBuilder::new()
    .post("http://code.ltpp.vip")
    .json(body)
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.decode(4096).text());
    }
    Err(e) => println!("Error => {}", e),
}
```

#### Send Body Text

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .post("http://ide.ltpp.vip/?language=rust")
    .text("hello")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.text());
    }
    Err(e) => println!("Error => {}", e),
}
```

#### Send Body Binary

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .post("http://ide.ltpp.vip/?language=rust")
    .body("hello".as_bytes())
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.decode(4096).text());
    }
    Err(e) => println!("Error => {}", e),
}
```

### Send request with HTTP proxy

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .http_proxy("127.0.0.1", 7890)
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.text());
    }
    Err(e) => println!("Error => {}", e),
}
```

### Send request with HTTP proxy authentication

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("https://ltpp.vip/")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .http_proxy_auth("127.0.0.1", 7890, "username", "password")
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.text());
    }
    Err(e) => println!("Error => {}", e),
}
```

### Send request with SOCKS5 proxy

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("http://ide.ltpp.vip/?language=rust")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .socks5_proxy("127.0.0.1", 1080)
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.text());
    }
    Err(e) => println!("Error => {}", e),
}
```

### Send request with SOCKS5 proxy authentication

```rs
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("header-key", "header-value");
let mut request_builder = RequestBuilder::new()
    .get("http://ide.ltpp.vip/?language=rust")
    .headers(header)
    .timeout(6000)
    .redirect()
    .max_redirect_times(8)
    .http1_1_only()
    .buffer(4096)
    .decode()
    .socks5_proxy_auth("127.0.0.1", 1080, "username", "password")
    .build_async();
match request_builder.send().await {
    Ok(response) => {
        println!("{:?}", response.text());
    }
    Err(e) => println!("Error => {}", e),
}
```

## Help

Ensure that CMake is installed on the system

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
