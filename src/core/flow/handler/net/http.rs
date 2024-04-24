use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::{Data, Node};

pub fn handle_net_http(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        "request_get" => {
            // 发起Get请求
            request_get(node, flow_data);
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
fn request_get(node: Node, flow_data: &Data) {}

// 发起Post请求
fn request_post(node: Node, flow_data: &Data) {}

// 发起Put请求
fn request_put(node: Node, flow_data: &Data) {}

// 发起Delete请求
fn request_delete(node: Node, flow_data: &Data) {}
