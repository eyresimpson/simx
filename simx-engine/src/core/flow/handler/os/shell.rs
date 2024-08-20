use crate::entity::flow::{FlowData, Node};

// 控制台/命令行相关（如打印）
pub fn handle_os_shell_println(node: Node, _flow_data: &mut FlowData) {
    println!("{}", node.attr.get("text").unwrap());
}