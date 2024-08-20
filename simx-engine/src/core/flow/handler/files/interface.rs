use crate::core::flow::handler::files::plain::handle_file_plain;
use crate::tools::log::interface::warn;
use simx_common::entity::flow::{FlowData, Node};

pub fn handle_file(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // zip相关（压缩、解压缩）
        "zip" => {}
        // 目录相关（列出目录、创建目录、删除目录、移动目录、重命名）
        "dir" => {}
        // 文件相关（创建、重命名、删除、移动）
        "file" => {}
        // 二进制文件（读取、写出、同步读取、同步写出）
        "binary" => {}
        // 普通文本（读取、写出、同步读取、同步写出）
        "plain" => handle_file_plain(node, flow_data),
        _ => {
            // 找不到，一般是用户写错了，或者设计器和引擎版本不兼容
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}