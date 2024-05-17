use crate::entity::flow::{Data, Node};

pub fn handle_db(node: Node, flow_data: &Data){
    println!("{:?},{:?}", node, flow_data)
}