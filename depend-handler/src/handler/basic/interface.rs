use crate::handler::basic::debug::handle_basic_debug;
use crate::handler::basic::flow::handle_basic_flow;
use crate::handler::basic::logic::handle_basic_logic;
use crate::handler::basic::variable::handle_basic_var;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;

pub fn handle_basic(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "debug" => {
            // debug，可以打印出当前流节点的数据
            handle_basic_debug(node, flow_data);
        }
        "flow" => {
            handle_basic_flow(node, flow_data);
        }
        "logic" => {
            handle_basic_logic(node, flow_data);
        }
        "var" => {
            handle_basic_var(node, flow_data);
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}