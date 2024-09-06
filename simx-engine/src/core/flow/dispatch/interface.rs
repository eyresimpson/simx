use crate::core::flow::exec::node::exec_node;
use crate::core::flow::resolver::interface::flow_resolver;
use engine_common::entity::flow::{Blueprint, Flow, FlowData, FlowRuntimeModel, FlowStatus};
use engine_common::logger::interface::{fail, info, success, warn};
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
        // TODO: 暂不实现，其实很简单，就是懒
        flow = Flow::default();
    }

    info(format!("flow {{ {} }} will be exec.", flow.flow_name).as_str());
    // 创建流运行时
    flow.runtime = Some(FlowRuntimeModel {
        status: FlowStatus::Starting,
        history: Default::default(),
        errors: Default::default(),
        warnings: Default::default(),
        messages: Default::default(),
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
                    // 执行开始的bp中对应的节点
                    let job = tokio::spawn(async move { dispatch_blueprint(key, bp) });
                    jobs.push(job);
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
                // del_flow_runtime(key);
                return;
            }
            FlowStatus::Error => {
                // 销毁掉内存中的流数据
                // del_flow_runtime(key);
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
        // TODO: 监听节点执行是否报错，如果有报错，就尝试执行补偿流
        exec_node(node, &mut flow_data).err().map(|e| {
            warn(format!("Node {} exec failed, error: {}", node_id, e).as_str());
            // 尝试执行补偿流
            // 检查补偿流列表是否为空
            if !redress_stream.is_empty() {
                for bp in redress_stream {
                    // 创建一个新线程并尝试执行节点
                    let key = key.clone();
                    tokio::spawn(async move { dispatch_blueprint(key, bp) });
                }
            }
        });
        // 将执行结束的数据写回到runtime中
        set_flow_runtime_flow_data(key_str, flow_data);
    }
    // 继续执行后续节点
    for bp in downstream {
        // 创建一个新线程并尝试执行节点
        let key = key.clone();
        tokio::spawn(async move { dispatch_blueprint(key, bp) });
    }
}
