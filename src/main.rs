use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use log::*;
use simple_logger;

fn main() {
    simple_logger::init().unwrap();
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 600];

    let s = stream.read(&mut buffer).unwrap();

    info!(
        "Request:\n\n{}\nRequest size: {}\n",
        String::from_utf8_lossy(&buffer[..]),
        s
    );
}
