use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{debug, warn};

pub fn handle_core_debug(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        "node" => {
            // 普通文本
            node_debug(node, flow_data)
        },
        "print" => {
            println!("{}", node.attr.get("text").unwrap());
            Ok(())
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 用于显示所有的节点内容，一般用于调试
fn node_debug(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    debug(format!("Debug Node: {:?}", node).as_str());
    debug(format!("Debug flow_data: {:?}", flow_data).as_str());
    Ok(())
}