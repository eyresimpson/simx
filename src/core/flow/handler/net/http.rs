use std::collections::HashMap;

use crate::core::common::log::interface::{fail, warn};
use crate::core::flow::entity::standardisation::{Data, Node};

pub async fn handle_net_http(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        "request_get" => {
            // 发起Get请求
            request_get(node, flow_data).await;
        }
        "request_post" => {
            // 发起Post请求
            request_post(node, flow_data);
        }
        "request_delete" => {
            // 发起Delete请求
            request_delete(node, flow_data);
        }
        "request_put" => {
            // 发起Put请求
            request_put(node, flow_data);
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

// 发起Get请求
async fn request_get(node: Node, flow_data: &mut Data) {
    println!("handle http get: {:?},{:?}", node, flow_data);
    let rest = reqwest::get("https://httpbin.org/ip").await.unwrap().json::<HashMap<String, String>>().await;
    if rest.is_err() {
        fail("Cannot launch new http get request.");
    } else {
        // flow_data.data.insert("res".to_string(), rest.unwrap().);
        println!("Res: {:?}", rest);
    }
    // 获取请求地址
    // let addr = node.attr.get("addr").unwrap();
    // // 获取请求端口
    // let port = node.attr.get("port").unwrap();
    // // 获取请求参数
    // let params = node.attr.get("params").unwrap();
    // // 获取请求头
    // let header = node.attr.get("header").unwrap();
    // let map = get("").await.unwrap();
    // let ret = to_string(&map).unwrap();
    // println!("===> {}", ret);
    // flow_data.data.insert("res".to_string(), ret);
}

// 发起Post请求
fn request_post(node: Node, flow_data: &Data) {
    println!("{:?},{:?}", node, flow_data)
}

// 发起Put请求
fn request_put(node: Node, flow_data: &Data) {
    println!("{:?},{:?}", node, flow_data)
}

// 发起Delete请求
fn request_delete(node: Node, flow_data: &Data) {
    println!("{:?},{:?}", node, flow_data)
}
