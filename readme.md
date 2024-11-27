## Rust http request

> Rust-HTTP-Request is a lightweight, efficient library for building, sending, and handling HTTP requests in Rust applications.

## Use

```rs
let http = HttpRequestBuilder::new()
    .set_methods(Methods::POST)
    .set_url("https://github.com/")
    .set_protocol(Protocol::HTTP)
    .set_body(&HashMap::new())
    .set_header(&HashMap::new())
    .builder();
let result: HttpRequest = _request_builder();
if let Ok(response) = result.send() {
    prinln!("{:?}", response);
}
```
