use engine_common::entity::error::NodeError;
use engine_common::entity::error::NodeError::HandleNotFound;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{fail, warn};
use evalexpr::eval_boolean;
use std::collections::HashMap;

pub fn handle_core_route(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 跳转语句
        "goto" => {
            warn("It is generally not recommended to use goto directly, because it may cause structural confusion or a dead loop.");
            goto(node, flow_data)
        }
        _ => {
            Err(HandleNotFound(node.handler))
        }
    }
}

// goto可以同时跳转到多个节点上（多线程或等待线程）
// TODO：有问题
fn goto(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    // 取表达式
    let router_str = node.attr.get("router").expect("cannot get router");
    let router_str = router_str.as_str().expect("router attr must be string");
    let routers: Vec<HashMap<String, String>> = serde_json::from_str(router_str).expect("cannot parse router, check your conf");
    for router in routers {
        // 取出表达式部分
        let expr = router.get("expression").expect("cannot get expr");
        // 取出目标节点
        let target = router.get("target").expect("cannot get target");
        // 执行跳跃判断
        jump(expr, target.to_string(), flow_data);
    }
    // Goto不会因为条件不匹配停止掉流，而是继续执行所有与其相连的其他节点（和if有区别）
    if flow_data.basics.route.is_empty() {
        // flow_data.basics.route = node.downstream;
    }
    Ok(())
}

// while循环
fn while_handle(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

// TODO: 有问题
fn jump(expr: &str, target: String, data: &mut FlowData) {
    let expr_rest: bool;

    // 作为bool解析表达式
    let eval_ret = eval_boolean(expr);

    if !eval_ret.is_err() {
        expr_rest = eval_ret.unwrap();
    } else {
        fail(format!("Cannot parse expression {}", expr).as_str());
        return;
    }

    if expr_rest {
        // data.basics.route.insert("", target.parse().unwrap());
    }
}