use crate::entity::standardisation::{Data, Node};

pub fn handle_db(node: Node, flow_data: &Data){
    println!("{:?},{:?}", node, flow_data)
}