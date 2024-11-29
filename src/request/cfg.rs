#[test]
fn test_post_request() {
    use crate::*;
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
}

#[test]
fn test_get_request() {
    use crate::*;
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
}
