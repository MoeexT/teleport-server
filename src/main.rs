use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use teleport_server::thread_pool::ThreadPool;

fn main() {
    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status, content) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "Hello World!")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404")
    };

    stream
        .write(format!("{}\r\n\r\n{}", status, content).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}
