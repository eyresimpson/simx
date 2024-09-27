use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;

pub fn handle_basic_var(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 创建/修改一个变量
        "set" => {}
        // 获取变量的值
        "get" => {}
        // 获取所有变量的Key
        "get_all" => {}
        // 删除变量（使其失效）
        "remove" => {}
        // 删除所有变量
        "remove_all" => {}
        // 复制变量
        "copy" => {}
        // 只读变量
        "readonly" => {}
        // 监听变量变化
        "watch" => {}
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}
