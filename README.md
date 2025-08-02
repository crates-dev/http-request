<center>

## http-request

[![](https://img.shields.io/crates/v/http-request.svg)](https://crates.io/crates/http-request)
[![](https://img.shields.io/crates/d/http-request.svg)](https://img.shields.io/crates/d/http-request.svg)
[![](https://docs.rs/http-request/badge.svg)](https://docs.rs/http-request)
[![](https://github.com/crates-dev/http-request/workflows/Rust/badge.svg)](https://github.com/crates-dev/http-request/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/http-request.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/HTTP-REQUEST/)

[Api Docs](https://docs.rs/http-request/latest/http_request/)

> A lightweight, efficient library for building, sending, and handling HTTP/HTTPS requests in Rust applications. http-request provides a simple and intuitive API, allowing developers to easily interact with web services, whether they use the "HTTP" or "HTTPS" protocol. The library supports various HTTP methods, custom headers, request bodies, timeout, automatic handling of redirects (including detecting redirect loops), and enhanced response body decoding (both automatic and manual), enabling fast and secure communication. Whether working with secure "HTTPS" connections or standard "HTTP" requests, the library is optimized for performance, minimal resource usage, and easy integration into Rust projects.

## Features

- **Support for HTTP/HTTPS**: Supports both HTTP and HTTPS protocols.
- **WebSocket Support**: Full WebSocket support with both synchronous and asynchronous APIs for real-time communication.
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
- **Proxy Support**: Comprehensive proxy support including HTTP, HTTPS, and SOCKS5 proxies with authentication for both HTTP requests and WebSocket connections.

## Installation

To use this crate, you can run cmd:

```shell
cargo add http-request
```

## Synchronous

### Send get request

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

### WebSocket connection

```rust
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("Authorization", "Bearer test-token");

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .headers(header)
    .timeout(10000)
    .buffer(4096)
    .protocols(&["chat", "superchat"])
    .build_sync();

websocket_builder
    .send_text("Hello WebSocket!")
    .and_then(|_| {
        println!("Sync WebSocket text message sent successfully");
        websocket_builder.send_binary(b"binary data")
    })
    .and_then(|_| {
        println!("Sync WebSocket binary message sent successfully");
        match websocket_builder.receive() {
            Ok(message) => match message {
                WebSocketMessage::Text(text) => println!("Received text: {}", text),
                WebSocketMessage::Binary(data) => println!("Received binary: {:?}", data),
                WebSocketMessage::Close => println!("Connection closed"),
                _ => println!("Received other message type"),
            },
            Err(e) => println!("Error receiving message: {}", e),
        }
        Ok(())
    })
    .and_then(|_| websocket_builder.close())
    .unwrap_or_else(|e| println!("Error => {}", e));
```

### WebSocket with HTTP proxy

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .http_proxy("127.0.0.1", 7890)
    .build_sync();

match websocket_builder.send_text("Hello WebSocket with HTTP proxy!") {
    Ok(_) => println!("WebSocket HTTP proxy message sent successfully"),
    Err(e) => println!("WebSocket HTTP proxy error: {}", e),
}
```

### WebSocket with HTTP proxy authentication

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .http_proxy_auth("127.0.0.1", 7890, "username", "password")
    .build_sync();

match websocket_builder.send_text("Hello WebSocket with HTTP proxy auth!") {
    Ok(_) => println!("WebSocket HTTP proxy auth message sent successfully"),
    Err(e) => println!("WebSocket HTTP proxy auth error: {}", e),
}
```

### WebSocket with SOCKS5 proxy

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .socks5_proxy("127.0.0.1", 1080)
    .build_sync();

match websocket_builder.send_text("Hello WebSocket with SOCKS5 proxy!") {
    Ok(_) => println!("WebSocket SOCKS5 proxy message sent successfully"),
    Err(e) => println!("WebSocket SOCKS5 proxy error: {}", e),
}
```

### WebSocket with SOCKS5 proxy authentication

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .socks5_proxy_auth("127.0.0.1", 1080, "username", "password")
    .build_sync();

match websocket_builder.send_text("Hello WebSocket with SOCKS5 proxy auth!") {
    Ok(_) => println!("WebSocket SOCKS5 proxy auth message sent successfully"),
    Err(e) => println!("WebSocket SOCKS5 proxy auth error: {}", e),
}
```

## Asynchronous

### Send get request

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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

### WebSocket connection

```rust
use http_request::*;

let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
header.insert("Authorization", "Bearer test-token");

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .headers(header)
    .timeout(10000)
    .buffer(4096)
    .protocols(&["chat", "superchat"])
    .build_async();

match websocket_builder.send_text_async("Hello WebSocket!").await {
    Ok(_) => {
        println!("Async WebSocket text message sent successfully");
        match websocket_builder.send_binary_async(b"binary data").await {
            Ok(_) => {
                println!("Async WebSocket binary message sent successfully");
                match websocket_builder.receive_async().await {
                    Ok(message) => match message {
                        WebSocketMessage::Text(text) => println!("Received text: {}", text),
                        WebSocketMessage::Binary(data) => println!("Received binary: {:?}", data),
                        WebSocketMessage::Close => println!("Connection closed"),
                        _ => println!("Received other message type"),
                    },
                    Err(e) => println!("Error receiving message: {}", e),
                }
            }
            Err(e) => println!("Error sending binary: {}", e),
        }
    }
    Err(e) => println!("Error sending text: {}", e),
}

websocket_builder
    .close_async_method()
    .await
    .unwrap_or_else(|e| println!("Error closing: {}", e));
```

### WebSocket with HTTP proxy

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .http_proxy("127.0.0.1", 7890)
    .build_async();

match websocket_builder.send_text_async("Hello WebSocket with HTTP proxy!").await {
    Ok(_) => println!("Async WebSocket HTTP proxy message sent successfully"),
    Err(e) => println!("Async WebSocket HTTP proxy error: {}", e),
}
```

### WebSocket with HTTP proxy authentication

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .http_proxy_auth("127.0.0.1", 7890, "username", "password")
    .build_async();

match websocket_builder.send_text_async("Hello WebSocket with HTTP proxy auth!").await {
    Ok(_) => println!("Async WebSocket HTTP proxy auth message sent successfully"),
    Err(e) => println!("Async WebSocket HTTP proxy auth error: {}", e),
}
```

### WebSocket with SOCKS5 proxy

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .socks5_proxy("127.0.0.1", 1080)
    .build_async();

match websocket_builder.send_text_async("Hello WebSocket with SOCKS5 proxy!").await {
    Ok(_) => println!("Async WebSocket SOCKS5 proxy message sent successfully"),
    Err(e) => println!("Async WebSocket SOCKS5 proxy error: {}", e),
}
```

### WebSocket with SOCKS5 proxy authentication

```rust
use http_request::*;

let mut websocket_builder: WebSocket = WebSocketBuilder::new()
    .connect("ws://127.0.0.1:60006/api/ws?uuid=1")
    .timeout(10000)
    .buffer(4096)
    .socks5_proxy_auth("127.0.0.1", 1080, "username", "password")
    .build_async();

match websocket_builder.send_text_async("Hello WebSocket with SOCKS5 proxy auth!").await {
    Ok(_) => println!("Async WebSocket SOCKS5 proxy auth message sent successfully"),
    Err(e) => println!("Async WebSocket SOCKS5 proxy auth error: {}", e),
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
