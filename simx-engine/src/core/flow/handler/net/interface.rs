use crate::core::flow::handler::net::http::handle_net_http;
use crate::tools::log::interface::warn;
use simx_common::entity::flow::{FlowData, Node};

pub async fn handle_net(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "http" => {
            handle_net_http(node, flow_data).await;
        }
        "https" => {}
        "tcp" => {}
        "udp" => {}

        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}