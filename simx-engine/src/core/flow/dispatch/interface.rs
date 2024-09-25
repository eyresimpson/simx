use crate::core::flow::exec::node::exec_node;
use crate::core::flow::resolver::interface::flow_resolver;
use engine_common::entity::flow::{
    Blueprint, Flow, FlowData, FlowRuntimeModel, FlowStatus, NodeTag, RouterItem,
};
use engine_common::logger::interface::{fail, info, success, warn};
use engine_common::runtime::config::get_simx_config;
use engine_common::runtime::flow::{
    del_flow_runtime, get_flow_runtime, get_flow_runtime_flow_data, get_flow_runtime_node_by_id,
    get_flow_runtime_status, set_flow_runtime, set_flow_runtime_flow_data, set_flow_runtime_status,
};
use std::path::Path;

// 调度执行流
// 此方法会根据流文件的path或json，生成Flow运行时并调度执行
// 第一个参数是流文件路径，第二个是流文件内容，两个参数必须有一个不为空
// 建议在新线程中执行此方法
pub async fn dispatch_flow(path: &Path, content: String) {
    let mut flow: Flow;
    if path.exists() && path.is_file() {
        // 加载流文件并解析为Flow对象
        flow = flow_resolver(path);
    } else {
        // 暂不实现
        flow = Flow::default();
    }

    info(format!("flow {{ {} }} will be exec.", flow.flow_name).as_str());
    // 创建流运行时
    flow.runtime = Some(FlowRuntimeModel {
        status: FlowStatus::Starting,
        history: Default::default(),

        // errors: Default::default(),
        // warnings: Default::default(),
        // messages: Default::default(),
        logs: vec![],
        current_node: None,
        data: FlowData {
            basics: Default::default(),
            params: Default::default(),
            data: Default::default(),
        },
    });
    // 将流运行时放入内存
    set_flow_runtime(path.to_str().unwrap(), flow.clone());
    // 尝试调度节点开始执行
    dispatch_node(path.to_str().unwrap()).await;

    success(format!("flow {{ {} }} has be exec success.", flow.flow_name).as_str());
}

// 调度执行节点
async fn dispatch_node(key: &str) {
    let engine_conf = get_simx_config().engine;
    // 调度最大重试次数
    let mut max_loop_count = 10;
    // 这五次是正常流转的损耗
    max_loop_count += 5;
    loop {
        if max_loop_count < 0 {
            warn("The maximum number of reoccurrences has been exceeded.");
            return;
        } else {
            max_loop_count -= 1;
        }
        // 分析流目前的状态，可以看作生命周期
        match get_flow_runtime_status(key) {
            FlowStatus::Starting => {
                set_flow_runtime_status(key, FlowStatus::Queue);
            }
            FlowStatus::Queue => {
                // 暂时不实现队列，直接执行
                set_flow_runtime_status(key, FlowStatus::Running);
            }
            FlowStatus::Running => {
                let blueprint = get_flow_runtime(key).unwrap().blueprint;

                let mut jobs = vec![];
                // 循环开始的blueprint列表（开始列表，入口节点群）
                for bp in blueprint {
                    let key = key.to_string();
                    // 根据配置进行控制
                    if engine_conf.blueprint_multi_entry_parallel {
                        let job = tokio::spawn(async move { dispatch_blueprint(key, bp) });
                        jobs.push(job);
                    } else {
                        // 串行执行
                        dispatch_blueprint(key, bp)
                    }
                    // 执行开始的bp中对应的节点（异步，同时执行所有的入口）
                }
                // 只要有一个流没有结束，就不退出运行状态
                for job in jobs {
                    // 只要有一个线程没有退出，就阻塞引擎不退出
                    job.await.unwrap();
                }
                set_flow_runtime_status(key, FlowStatus::Finished);
            }
            FlowStatus::Finished => {
                // 销毁掉内存中的流数据
                del_flow_runtime(key);
                return;
            }
            FlowStatus::Error => {
                // 销毁掉内存中的流数据
                del_flow_runtime(key);
                return;
            }
            // 暂不实现
            FlowStatus::Paused => {}
            // 暂不处理
            FlowStatus::Waiting => {}
            // 暂不处理
            FlowStatus::Unknown => {
                // fail("flow runtime status is unknown!");
                fail("flow runtime status is unknown!");
                return;
            }
        }
    }
}

// 调度执行节点
pub fn dispatch_blueprint(key: String, blueprint: Blueprint) {
    // let engine_conf = get_simx_config().engine;
    let key_str = key.as_str();
    // 当前节点id
    let node_id = blueprint.node.as_str();
    // 下游节点群
    let downstream = blueprint.downstream;
    // 补偿流
    let redress_stream = blueprint.redress_stream;
    let mut flow_data = get_flow_runtime_flow_data(key_str);
    // 尝试根据id获取对应的流节点
    if let Some(node) = get_flow_runtime_node_by_id(key_str, node_id) {
        // 执行流节点
        exec_node(node.clone(), &mut flow_data).err().map(|e| {
            warn(format!("Node {} exec failed, error: {}", node_id, e).as_str());
            // 尝试执行补偿流
            // 检查补偿流列表是否为空
            if !redress_stream.is_empty() {
                info(format!("Node {} try to exec redress stream.", node_id).as_str());
                for bp in redress_stream {
                    let key = key.clone();
                    dispatch_blueprint(key, bp)
                }
            }
        });
        // 将执行结束的数据写回到runtime中
        set_flow_runtime_flow_data(key_str, flow_data);

        // 分析节点类型，如果是逻辑节点，就根据逻辑节点的struct确定要流出到哪些downstream，如果不是，则流出到所有的downstream
        if node.tags.is_none() || !node.tags.unwrap().contains(&NodeTag::Logic) {
            // 非逻辑节点
            for bp in downstream {
                let key = key.clone();
                dispatch_blueprint(key, bp)
            }
        } else {
            let mut downstream = downstream.clone();
            // 如果是goto节点
            // TODO：后续考虑限制甚至删除goto，可能过于自由了，太容易形成死循环了
            if node.handler.eq("simx.goto") {
                let downstream_str = node.attr.get("router").unwrap();
                if downstream_str.is_empty() {
                    // Goto节点的路由为空，直接退出就行
                    warn("Goto node must have a router attribute.");
                    return;
                }

                let router: Vec<RouterItem> = serde_json::from_str(downstream_str).unwrap();
                // for item in router {
                // 遍历路由表，找到匹配的节点，并加入到downstream中
                // if let Some(bp) = downstream.iter().find(|bp| bp.node.eq(&item.target)) {
                // 找到匹配的节点，加入到downstream中
                // let mut new_bp = bp.clone();
                // 尝试根据表达式计算，如果计算失败，则直接跳过
                // let result = eval(item.expression.as_str(), &flow_data);

                // }
                // }
                // downstream = serde_json::from_str(downstream_str).unwrap();
            }
            // 逻辑节点
            // TODO: 临时测试，继续执行后续节点
            for bp in downstream {
                // goto是个特例，这个节点可以直接跳转到目标，而不需要连线（也就是downstream中可能没有记录）
                let key = key.clone();
                dispatch_blueprint(key, bp)
            }
        }
    }
}
