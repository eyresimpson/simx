use crate::core::flow::dispatch::common::match_node_id;
use crate::core::flow::dispatch::interface::dispatch_nodes;
use engine_common::entity::error::DispatchErr;
use engine_common::entity::flow::{Blueprint, FlowData, Node};
use engine_common::logger::interface::fail;
use evalexpr::eval_boolean;

pub async fn dispatch_loop(blueprint: Blueprint, node: Node, mut flow_data: &mut FlowData) -> Result<(), DispatchErr> {
    // 取条件
    let expression = node.attr.get("expression").expect("expression must be set");
    // 取开始列表
    let endpoints = node.attr.get("endpoints").expect("endpoints must be set").as_array().expect("endpoints must be array object");
    let expression = node.attr.get("expression").expect("expression must be set");
    // 取调度等待间隔
    let interval: f64 = match node.attr.get("interval") {
        None => {
            // 为-1就不等待，无间隙
            -1f64
        }
        Some(interval) => {
            interval.as_f64().unwrap()
        }
    };
    // 是否异步开始
    let parallel_endpoints: bool = match node.attr.get("parallel_endpoints") {
        None => {
            false
        }
        Some(parallel_endpoints) => {
            parallel_endpoints.as_bool().unwrap()
        }
    };
    // 取调度等待间隔
    let parallel_routes: bool = match node.attr.get("parallel_routes") {
        None => {
            // 为-1就不等待，无间隙
            false
        }
        Some(parallel_routes) => {
            parallel_routes.as_bool().unwrap()
        }
    };
    // 准备用于子流程的蓝图
    let mut blueprint: Blueprint = blueprint.clone();
    blueprint.endpoints.clear();
    blueprint.parallel_endpoints = parallel_endpoints;
    blueprint.parallel_routes = parallel_routes;

    let node_id = node.clone().id.expect("node id must be set");
    // 取目标路由数组
    // let router = node.attr.get("router").expect("cannot get expr").as_array().expect("router must be array object");
    // 取最大重试次数
    let mut loop_restriction: i64;
    // 首先判断node中有没有数据
    match flow_data.nodes.get(&node_id) {
        Some(value) => {
            loop_restriction = value.as_i64().unwrap();
        }
        None => {
            loop_restriction = match node.attr.get("loop_restriction") {
                Some(value) => {
                    value.as_i64().unwrap()
                }
                None => {
                    // TODO：这个默认值应该在配置文件中写明
                    30
                }
            }
        }
    }


    // 执行表达式
    let mut result = match eval_boolean(expression.as_str().unwrap()) {
        Ok(value) => { value }
        Err(err) => {
            fail(format!("Cannot parse expression {}, err: {}", expression, err).as_str());
            false
        }
    };

    let loop_result: Result<(), DispatchErr>;

    // 循环所有的开始节点
    for node_id in endpoints {
        let node = match match_node_id(node_id, &blueprint) {
            Ok(node) => { node }
            Err(err) => { return Err(err) }
        };
        // TODO：这个判断在并发下非常危险，临时使用
        while result && loop_restriction != 0 {
            // 执行表达式
            result = match eval_boolean(expression.as_str().unwrap()) {
                Ok(value) => { value }
                Err(err) => {
                    fail(format!("Cannot parse expression {}, err: {}", expression, err).as_str());
                    false
                }
            };
            println!("result: {}, loop_restriction: {}", result, loop_restriction);

            // 执行跳转逻辑
            dispatch_nodes(blueprint.clone(), node.clone(), flow_data).await.unwrap();
            loop_restriction -= 1;
            println!("loop_restriction: {}", loop_restriction);
            // 执行等待间隙
            if interval != -1f64 {
                tokio::time::sleep(std::time::Duration::from_secs_f64(interval)).await;
            }
        }
    }
    if loop_restriction == 0 {
        fail("Loop run over limit.");
        return Err(DispatchErr::RunOverLimit);
    }
    Ok(())
}