use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::{Data, Node};
use crate::core::flow::handler::net::http::handle_net_http;

pub fn handle_net(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "http" => {
            handle_net_http(node, flow_data);
        }
        "https" => {}
        "tcp" => {}
        "udp" => {}

        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}