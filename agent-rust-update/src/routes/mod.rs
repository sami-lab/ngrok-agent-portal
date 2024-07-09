use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use crate::controller::{get_hello, post_hello};

pub fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if !request.is_empty() {
        let first_line = &request[0];
        if first_line.starts_with("GET / HTTP/1.1") {
            handle_get_request(stream);
        } else if first_line.starts_with("POST / HTTP/1.1") {
            handle_post_request(stream, &request);
        } else {
            handle_404(&mut stream);
        }
    } else {
        handle_404(&mut stream);
    }
}

fn handle_get_request(mut stream: TcpStream) {
    let response = get_hello();

    let status_line = "HTTP/1.1 200 OK";
    let length = response.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{response}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_post_request(mut stream: TcpStream, request: &[String]) {
    // Collect the body of the POST request
    let body = request.iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    post_hello(&mut stream, &body);
}

fn handle_404(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = "404 Not Found";
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
