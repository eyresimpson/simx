use crate::handler::files::format::plain::handle_file_plain;
use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};

pub fn handle_file_format(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // zip相关（压缩、解压缩）
        "zip" => { Ok(()) }
        // 目录相关（列出目录、创建目录、删除目录、移动目录、重命名）
        "dir" => { Ok(()) }
        // 文件相关（创建、重命名、删除、移动）
        "file" => { Ok(()) }
        // 二进制文件（读取、写出、同步读取、同步写出）
        "binary" => { Ok(()) }
        // 普通文本（读取、写出、同步读取、同步写出）
        "plain" => handle_file_plain(node, flow_data),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}