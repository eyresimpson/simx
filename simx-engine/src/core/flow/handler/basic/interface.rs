use crate::core::flow::handler::basic::debug::handle_basic_debug;
use crate::core::flow::handler::basic::flow::handle_basic_flow;
use crate::tools::log::interface::warn;
use simx_common::entity::flow::{FlowData, Node};

pub async fn handle_basic(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "debug" => {
            // debug，可以打印出当前流节点的数据
            handle_basic_debug(node, flow_data);
        }
        "flow" => {
            handle_basic_flow(node, flow_data).await;
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}