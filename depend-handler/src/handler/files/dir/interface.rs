use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;

pub fn handle_files_dir(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建目录
        "create" => {}
        // 判断目录是否存在
        "exist" => {}
        // 移动目录
        "mv" => {}
        // 复制目录
        "cp" => {}
        // 目录授权
        "chmod" => {}
        // 删除目录
        "del" => {}
        _ => {
            // 找不到，一般是用户写错了，或者设计器和引擎版本不兼容
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}