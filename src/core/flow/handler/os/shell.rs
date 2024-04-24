use std::collections::HashMap;
use crate::core::flow::entity::standardisation::{Data, Node};

// 控制台/命令行相关（如打印）
pub fn handle_os_shell(node: Node, flow_data: &mut Data) {
    println!("{}", node.attr.get("text").unwrap());
}