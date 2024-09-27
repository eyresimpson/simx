use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;

pub fn handle_core_var(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 创建/修改一个变量
        "set" => {
            // 判断用户也没有写进去变量数据，有可能编辑器没有拦截
            if node.attr.get("var_name").is_some() && node.attr.get("var_value").is_some() {
                let key = node.attr.get("var_name").unwrap().clone();
                let val = node.attr.get("var_value").unwrap().clone();
                flow_data.params.insert(key, val);
            } else {
                warn("Cannot find variable name, Skip...");
            }
        }
        // 删除变量（使其失效）
        "remove" => {
            // 判断用户也没有写进去变量数据，有可能编辑器没有拦截
            if node.attr.get("var_name").is_some() {
                let key = node.attr.get("var_name").unwrap().clone();
                if flow_data.params.get(&key).is_some() {
                    flow_data.params.remove(&key);
                } else {
                    warn(format!("Cannot find variable by {}, Skip...", key).as_str());
                }
            } else {
                warn("Cannot find variable name, Skip...");
            }
        }
        // 删除所有变量
        "remove_all" => {
            flow_data.params.clear();
        }

        // 监听变量变化
        "watch" => {}

        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}
