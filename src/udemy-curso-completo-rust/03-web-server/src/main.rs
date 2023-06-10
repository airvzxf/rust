use std::{
    fs::read_to_string,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("# Start server");
    let address = "127.0.0.1:8000";
    println!("address: {}", address);
    let listener: TcpListener = TcpListener::bind(&address).unwrap();
    println!("listener: {:#?}", listener);

    println!("# Listening connections");
    for incoming in listener.incoming() {
        println!("incoming: {:#?}", incoming);
        let stream: TcpStream = incoming.unwrap();
        println!("Stream: {:#?}", stream);

        println!("# Handle connections");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Stream received.");
    println!("buffer: {:#?}", String::from_utf8_lossy(&buffer[..]));

    if buffer.starts_with(b"GET / HTTP/1.1") {
        send_index(stream);
    } else {
        send_not_found(stream);
    }
}

fn send_not_found(stream: TcpStream) {
    println!("# Response to the client: Not Found");
    let filename = "404.html";
    let status = "404 Not Found";
    send_to_client(stream, filename, status);
}

fn send_index(stream: TcpStream) {
    println!("# Response to the client: Index Ok");
    let filename = "index.html";
    let status = "200 OK";
    send_to_client(stream, filename, status);
}

fn send_to_client(mut stream: TcpStream, filename: &str, status: &str) {
    let content_page = read_to_string(filename).unwrap();
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        content_page.len(),
        content_page
    );

    stream.write(&response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
