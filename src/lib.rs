use std::collections::HashMap;

use hyper::{Body, Request};

#[no_mangle]
pub fn on_request(request: Request<Body>, addr: String, cache: HashMap<String, String>) -> (Request<Body>, HashMap<String, String>) {
    println!("on_request: {} {}", request.method(), request.uri());
    println!("addr_on_request: {}", addr);
    return (request, cache);
}

#[no_mangle]
pub fn on_response(request: Request<Body>, addr: String, cache: HashMap<String, String>) -> (Request<Body>, HashMap<String, String>) {
    println!("on_response: {} {}", request.method(), request.uri());
    println!("addr_on_response: {}", addr);
    return (request, cache);
}