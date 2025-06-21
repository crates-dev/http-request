#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_https_over_http_proxy_async() {
        let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
        header.insert("User-Agent", "test-agent");

        let mut request_builder = RequestBuilder::new()
            .get("https://httpbin.org/get")
            .headers(header)
            .timeout(10000)
            .http_proxy("proxy.example.com", 8080)
            .build_async();

        match request_builder.send().await {
            Ok(response) => {
                println!(
                    "HTTPS over HTTP proxy test passed: {}",
                    response.binary().get_status_code()
                );
            }
            Err(e) => {
                println!("HTTPS over HTTP proxy test failed (expected): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_https_over_socks5_proxy_async() {
        let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
        header.insert("User-Agent", "test-agent");

        let mut request_builder = RequestBuilder::new()
            .get("https://httpbin.org/get")
            .headers(header)
            .timeout(10000)
            .socks5_proxy("127.0.0.1", 1080)
            .build_async();

        match request_builder.send().await {
            Ok(response) => {
                println!(
                    "HTTPS over SOCKS5 proxy test passed: {}",
                    response.binary().get_status_code()
                );
            }
            Err(e) => {
                println!("HTTPS over SOCKS5 proxy test failed (expected): {}", e);
            }
        }
    }

    #[test]
    fn test_https_over_http_proxy_sync() {
        let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
        header.insert("User-Agent", "test-agent");

        let mut request_builder = RequestBuilder::new()
            .get("https://httpbin.org/get")
            .headers(header)
            .timeout(10000)
            .http_proxy("proxy.example.com", 8080)
            .build();

        match request_builder.send() {
            Ok(response) => {
                println!(
                    "Sync HTTPS over HTTP proxy test passed: {}",
                    response.binary().get_status_code()
                );
            }
            Err(e) => {
                println!("Sync HTTPS over HTTP proxy test failed (expected): {}", e);
            }
        }
    }

    #[test]
    fn test_https_over_socks5_proxy_sync() {
        let mut header: HashMapXxHash3_64<&str, &str> = hash_map_xx_hash3_64();
        header.insert("User-Agent", "test-agent");

        let mut request_builder = RequestBuilder::new()
            .get("https://httpbin.org/get")
            .headers(header)
            .timeout(10000)
            .socks5_proxy("127.0.0.1", 1080)
            .build();

        match request_builder.send() {
            Ok(response) => {
                println!(
                    "Sync HTTPS over SOCKS5 proxy test passed: {}",
                    response.binary().get_status_code()
                );
            }
            Err(e) => {
                println!("Sync HTTPS over SOCKS5 proxy test failed (expected): {}", e);
            }
        }
    }
}
