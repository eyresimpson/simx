use crate::core::flow::entity::standardisation::{Data, Node};

pub fn handle_file_xml(node: Node, flow_data: &Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        "reader" => {
            // 普通文本
        }
        "writer" => {
            // Json文件
        }
        _ => {}
    }
}