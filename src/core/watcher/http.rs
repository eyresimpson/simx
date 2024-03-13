use std::net::TcpListener;
use std::thread;

use crate::tools::log::shell::success;

pub fn start_http_watcher() {
    let listener = TcpListener::bind("127.0.0.1:18000").unwrap();
    success("System listening on port 18000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    crate::tools::net::http::handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}