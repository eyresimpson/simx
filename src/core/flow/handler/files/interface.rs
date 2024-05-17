use crate::core::common::log::interface::warn;
use crate::core::flow::handler::files::plain::handle_file_plain;
use crate::entity::standardisation::{Data, Node};

pub fn handle_file(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "plain" => {
            // 普通文本
            handle_file_plain(node, flow_data);
        }
        "json" => {
            // Json文件
        }
        "xml" => {
            // XML文件
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}