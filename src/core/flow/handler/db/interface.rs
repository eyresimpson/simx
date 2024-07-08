use crate::entity::flow::{FlowData, Node};

pub fn handle_db(node: Node, flow_data: &FlowData) {
    println!("{:?},{:?}", node, flow_data)
}