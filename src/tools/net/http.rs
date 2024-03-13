use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    // Check if it's a GET or POST request
    if request.starts_with("GET") {
        println!("GET Request:");
    } else if request.starts_with("POST") {
        println!("POST Request:");
    } else {
        println!("Unsupported Request:");
    }

    // Print the parameters
    println!("{}", request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}