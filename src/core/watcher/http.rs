use std::net::TcpListener;
use std::thread;
use crate::conf::simx::get_config;

use crate::tools::log::shell::{err, success};

pub fn start_http_watcher() {
    let conf = get_config();
    // 获取监听地址
    let addr = conf.get("net").unwrap().get("http-listener-address").unwrap().as_str().unwrap();
    // 获取监听端口
    let port = conf.get("net").unwrap().get("http-listener-port").unwrap().as_integer().unwrap();
    // 创建绑定
    let listener = TcpListener::bind(format!("{}:{}", addr, port).as_str()).unwrap();
    success(format!("System Http Watcher Run on {}:{}", addr, port).as_str());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    crate::tools::net::http::handle_client(stream);
                });
            }
            Err(e) => {
                err(format!("Start http watcher with Err:\n{}", e).as_str());
            }
        }
    }
}