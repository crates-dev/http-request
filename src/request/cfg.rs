#[test]
fn test_request() {
    use crate::*;
    let mut _request_builder = HttpRequestBuilder::new()
        .set_methods(Methods::POST)
        .set_url("http://localhost:8080/")
        .set_body(&HashMap::new())
        .set_header(&HashMap::new())
        .builder();
    if let Ok(response) = _request_builder.send() {
        println!("{:?}", response);
    }
}
