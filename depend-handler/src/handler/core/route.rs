use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{debug, warn};
use std::collections::HashMap;

pub fn handle_core_route(node: Node, flow_data: &mut FlowData) {
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

// goto可以同时跳转到多个节点上（多线程或等待线程）
fn goto(node: Node, flow_data: &mut FlowData){
    // 取表达式
    let router_str = node.attr.get("router").expect("cannot get router");
    let routers: Vec<HashMap<String, String>> = serde_json::from_str(router_str).expect("cannot parse router, check your conf");
    for router in routers {
        // 取出表达式部分
        let expr = router.get("expression").expect("cannot get expr");
        // 取出目标节点
        let target = router.get("target").expect("cannot get target");
        // 这里应该处理下表达式的
        if expr == "true" {
            // flow_data.basics.downstream.clear();
            flow_data.basics.downstream.push(target.to_string());
        }
    }
}