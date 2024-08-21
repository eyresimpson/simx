use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;

pub fn handle_basic_debug(node: Node, flow_data: &mut FlowData) {
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
#[allow(unused_variables)]
fn node_debug(node: Node, flow_data: &mut FlowData) {
    // let f = flow_data.data.get("text").unwrap();
    // warn(f.)
}