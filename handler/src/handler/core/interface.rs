use crate::handler::core::debug::handle_core_debug;
use crate::handler::core::flow::handle_core_flow;
use crate::handler::core::variable::handle_core_var;
use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};

pub fn handle_core(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "debug" =>
            handle_core_debug(node, flow_data),

        "flow" =>
            handle_core_flow(node, flow_data),

        "var" =>
            handle_core_var(node, flow_data),

        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}