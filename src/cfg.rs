use crate::*;
use std::{
    sync::Mutex,
    thread::{JoinHandle, spawn},
    time::Instant,
};

#[test]
fn test_http_post_request() {
    let header: JsonValue = json_value!({
        "Accept": "*/*",
        "Content-Type": "application/json",
        "Connection": "keep-alive",
        "Accept-Encoding": "gzip, deflate"
    });
    let body: JsonValue = json_value!({
        "code": "hello",
        "language": "rust",
        "testin": ""
    });
    let mut request_builder = RequestBuilder::new()
        .post("http://localhost:80/rust?hello=rust")
        .json(body)
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_http_get_request() {
    let header: JsonValue = json_value!({
        "Accept": "*/*",
        "Content-Type": "application/json",
        "Connection": "keep-alive",
        "Accept-Encoding": "gzip, deflate"
    });
    let mut request_builder = RequestBuilder::new()
        .get("http://localhost")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_https_post_request() {
    let header: JsonValue = json_value!({
        "Accept": "*/*",
        "Content-Type": "application/json",
        "Connection": "keep-alive",
        "Accept-Encoding": "gzip, deflate"
    });
    let body: JsonValue = json_value!({
        "code": "fn main() {\r\n    println!(\"hello world\");\r\n}",
        "language": "rust",
        "testin": "",
    });
    let mut request_builder = RequestBuilder::new()
        .post("https://code.ltpp.vip/")
        .json(body)
        .headers(header)
        .timeout(4000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_https_get_request() {
    let header: JsonValue = json_value!({
        "Accept": "*/*",
        "Content-Type": "application/json",
        "Connection": "keep-alive",
        "Accept-Encoding": "gzip, deflate"
    });
    let mut request_builder = RequestBuilder::new()
        .get("https://code.ltpp.vip/")
        .headers(header)
        .timeout(4000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_http_post_text_request() {
    let header: JsonValue = json_value!({
        "Accept": "*/*",
        "Content-Type": "application/json",
        "Connection": "keep-alive",
        "Accept-Encoding": "gzip, deflate"
    });
    let mut request_builder = RequestBuilder::new()
        .post("http://localhost:80")
        .text("hello")
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_http_post_binary_request() {
    let header: JsonValue = json_value!({
        "Accept": "*/*",
        "Content-Type": "application/json",
        "Connection": "keep-alive",
        "Accept-Encoding": "gzip, deflate"
    });
    let mut request_builder = RequestBuilder::new()
        .post("http://localhost:80")
        .body("hello".as_bytes())
        .headers(header)
        .timeout(6000)
        .redirect()
        .buffer(4096)
        .max_redirect_times(8)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_auto_gzip_get() {
    let mut request_builder = RequestBuilder::new()
        .get("https://proxy.ltpp.vip/visit/add?origin=https://docs.ltpp.vip/")
        .timeout(4000)
        .redirect()
        .max_redirect_times(8)
        .decode()
        .buffer(4096)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_gzip_get() {
    let mut request_builder = RequestBuilder::new()
        .get("https://proxy.ltpp.vip/visit/add?origin=https://docs.ltpp.vip/")
        .timeout(4000)
        .redirect()
        .max_redirect_times(8)
        .buffer(4096)
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.decode(4096).text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_unredirect_get() {
    let mut request_builder = RequestBuilder::new()
        .post("https://proxy.ltpp.vip/visit/add?origin=https://docs.ltpp.vip/")
        .timeout(4000)
        .max_redirect_times(8)
        .buffer(4096)
        .unredirect()
        .http1_1_only()
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {}", e));
}

#[test]
fn test_thread_https_get_request() {
    let num_threads: i32 = 10;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let header: JsonValue = json_value!({
        "Accept": "*/*",
        "Content-Type": "application/json",
        "Connection": "keep-alive",
        "Accept-Encoding": "gzip, deflate"
    });
    let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .get("https://code.ltpp.vip/")
            .headers(header.clone())
            .timeout(4000)
            .redirect()
            .buffer(4096)
            .max_redirect_times(8)
            .http1_1_only()
            .build(),
    ));
    for _ in 0..num_threads {
        let request_builder: Arc<
            Mutex<
                Box<
                    dyn RequestTrait<
                        RequestResult = Result<
                            Box<
                                dyn ResponseTrait<
                                        OutputText = HttpResponseText,
                                        OutputBinary = HttpResponseBinary,
                                    >,
                            >,
                            RequestError,
                        >,
                    >,
                >,
            >,
        > = Arc::clone(&request_builder);
        let handle: JoinHandle<()> = spawn(move || {
            let mut request_builder: std::sync::MutexGuard<
                '_,
                Box<
                    dyn RequestTrait<
                        RequestResult = Result<
                            Box<
                                dyn ResponseTrait<
                                        OutputText = HttpResponseText,
                                        OutputBinary = HttpResponseBinary,
                                    >,
                            >,
                            RequestError,
                        >,
                    >,
                >,
            > = request_builder.lock().unwrap();
            let start_time: Instant = Instant::now();
            match request_builder.send() {
                Ok(response) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    let response_text: HttpResponseText = response.text();
                    println!("Thread finished in: {:?}", duration);
                    println!("ResponseTrait => {:?}", response_text);
                }
                Err(e) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    println!("Thread finished in: {:?}", duration);
                    println!("Error => {}", e);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_thread_http_get_request() {
    let num_threads: i32 = 10;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .get("http://127.0.0.1:8080/")
            .timeout(10)
            .redirect()
            .buffer(100)
            .max_redirect_times(0)
            .http2_only()
            .build(),
    ));
    for _ in 0..num_threads {
        let request_builder: Arc<
            Mutex<
                Box<
                    dyn RequestTrait<
                        RequestResult = Result<
                            Box<
                                dyn ResponseTrait<
                                        OutputText = HttpResponseText,
                                        OutputBinary = HttpResponseBinary,
                                    >,
                            >,
                            RequestError,
                        >,
                    >,
                >,
            >,
        > = Arc::clone(&request_builder);
        let handle: JoinHandle<()> = spawn(move || {
            let mut request_builder = request_builder.lock().unwrap();
            let start_time: Instant = Instant::now();
            match request_builder.send() {
                Ok(response) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    println!("Thread finished in: {:?}", duration);
                    let response_text: HttpResponseText = response.text();
                    println!("ResponseTrait => {:?}", response_text);
                }
                Err(e) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    println!("Thread finished in: {:?}", duration);
                    println!("Error => {}", e);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
