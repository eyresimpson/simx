use engine_common::entity::flow::{FlowData, Node};

pub fn handler_http(node: Node, flow_data: &mut FlowData) {
    match node.handler.as_str() {
        "get" => { request_http(node, flow_data) }
        "post" => {}
        "put" => {}
        "delete" => {}
        "request" => {}
        _ => {}
    }
}

fn request_http(node: Node, flow_data: &mut FlowData) {
    println!("{:?}", node);
    println!("{:?}", flow_data);
}
