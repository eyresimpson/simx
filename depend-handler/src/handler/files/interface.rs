use crate::handler::files::dir::interface::handle_files_dir;
use crate::handler::files::file::interface::handle_files_file;
use crate::handler::files::format::interface::handle_file_format;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;

pub fn handle_file(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // 目录相关（列出目录、创建目录、删除目录、移动目录、重命名）
        "dir" => handle_files_dir(node, flow_data),
        // 文件相关（创建、重命名、删除、移动）
        "file" => handle_files_file(node, flow_data),
        // 格式文件处理（读取、写出、同步读取、同步写出）
        "format" => handle_file_format(node, flow_data),
        // Json文件处理
        "json" => {}
        // Xml文件处理
        "xml" => {}
        // Toml文件处理
        "toml" => {}
        // Yaml文件处理
        "yaml" => {}
        _ => {
            // 找不到，一般是用户写错了，或者设计器和引擎版本不兼容
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}