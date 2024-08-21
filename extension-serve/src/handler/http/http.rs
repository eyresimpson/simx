use engine_common::entity::flow::{FlowData, Node};

#[allow(unused_variables)]
pub fn handler_http(node: Node, flow_data: &mut FlowData) {
    match node.handler.as_str() {
        // "root" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        _ => {}
    }
}