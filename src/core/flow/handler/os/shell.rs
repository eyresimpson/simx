use crate::entity::flow::{Data, Node};

// 控制台/命令行相关（如打印）
pub fn handle_os_shell(node: Node, _flow_data: &mut Data) {
    // flow_data;
    println!("{}", node.attr.get("text").unwrap());
}