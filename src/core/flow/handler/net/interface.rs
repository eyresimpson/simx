use crate::core::common::log::interface::warn;
use crate::core::flow::handler::net::http::handle_net_http;
use crate::entity::standardisation::{Data, Node};

pub async fn handle_net(node: Node, flow_data: &mut Data) {
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