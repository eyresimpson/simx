use crate::core::common::log::interface::warn;
use crate::core::flow::handler::basic::debug::handle_basic_debug;
use crate::entity::standardisation::{Data, Node};

pub fn handle_basic(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "debug" => {
            // debug
            handle_basic_debug(node, flow_data)
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}