use toml::to_string;

use crate::core::common::log::interface::{debug, warn};
use crate::entity::flow::{Data, Node};

pub fn handle_basic_debug(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        "node" => {
            // 普通文本
            node_debug(node, flow_data)
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

// 用于显示所有的节点内容，一般用于调试
fn node_debug(node: Node, flow_data: &mut Data) {
    debug(format!("Node: {}", to_string(&node).unwrap().as_str()).as_str());
    debug(format!("Data: {}", to_string(&flow_data).unwrap().as_str()).as_str());
}