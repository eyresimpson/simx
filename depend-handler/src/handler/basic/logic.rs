use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{debug, warn};

pub fn handle_basic_logic(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 条件语句
        "if" => {}
        // 否则语句
        "else" => {}
        // 否则判断语句
        "else_if" => {}
        // 循环语句
        "for" => {}
        // 数组循环
        "foreach" => {}
        // 跳转语句
        "goto" => {
            debug("It is generally not recommended to use goto directly, because it may cause structural confusion or a dead loop.");
            goto(node, flow_data)
        }
        // 选择语句
        "switch" => {}
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

fn goto(node: Node, flow_data: &mut FlowData){
    // 获取要跳转到哪一个节点
    
    
}