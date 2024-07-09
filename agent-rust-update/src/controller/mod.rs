use std::io::prelude::*;
use std::net::TcpStream;

pub fn get_hello() -> String {
    "Hello, world!".to_string()
}

pub fn post_hello(stream: &mut TcpStream, body: &str) {
    println!("Received POST request with body: {}", body);

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", "Post request received");
    stream.write_all(response.as_bytes()).unwrap();
}
