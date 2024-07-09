use std::net::TcpListener;
use std::thread;
use std::sync::Arc;
use crate::routes::handle_request;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let listener = Arc::new(listener);

    println!("Server is running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let listener = Arc::clone(&listener);

        thread::spawn(move || {
            handle_request(stream);
        });
    }
}
