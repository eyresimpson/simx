use crate::handler::core::debug::handle_core_debug;
use crate::handler::core::flow::handle_core_flow;
use crate::handler::core::route::handle_core_route;
use crate::handler::core::variable::handle_core_var;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;

pub fn handle_core(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "debug" => {
            // debug，可以打印出当前流节点的数据
            handle_core_debug(node, flow_data);
        }
        "flow" => {
            handle_core_flow(node, flow_data);
        }
        "route" => {
            handle_core_route(node, flow_data);
        }
        "var" => {
            handle_core_var(node, flow_data);
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}