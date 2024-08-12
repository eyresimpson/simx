use crate::core::common::log::interface::warn;
use crate::core::flow::handler::os::shell::handle_os_shell_println;
use crate::entity::flow::{FlowData, Node};

pub fn handle_os(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "shell" => {
            // 控制台操作
            handle_os_shell_println(node, flow_data);
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}
