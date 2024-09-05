use crate::core::flow::resolver::interface::flow_resolver;
use engine_common::entity::flow::{FlowData, FlowRuntimeModel, FlowStatus, NodeTag};
use engine_common::logger::interface::fail;
use engine_common::runtime::flow::{get_flow_runtime, get_flow_runtime_nodes, get_flow_runtime_queue, get_flow_runtime_status, push_flow_runtime_queue, set_flow_runtime, set_flow_runtime_status};
use std::path::Path;

// 调度执行流
// 此方法会根据流文件的path或json，生成Flow运行时并调度执行
// 第一个参数是流文件路径，第二个是流文件内容，两个参数必须有一个不为空
// 建议在新线程中执行此方法
pub fn dispatch_flow(path: &Path, content: String) {
    // 加载流文件并解析为Flow对象
    let mut flow = flow_resolver(path);

    // 创建流运行时
    flow.runtime = Some(FlowRuntimeModel {
        status: FlowStatus::Starting,
        history: Default::default(),
        errors: Default::default(),
        warnings: Default::default(),
        messages: Default::default(),
        current_node: None,
        queue: Default::default(),
        data: FlowData {
            basics: Default::default(),
            params: Default::default(),
            data: Default::default(),
        },
    });
    // 将流运行时放入内存
    set_flow_runtime(path.to_str().unwrap(), flow.clone());
    // 尝试调度节点开始执行
    dispatch_node(path.to_str().unwrap());
}

// 调度执行节点
fn dispatch_node(key: &str) {
    // 节点列表
    let mut max_loop_count = 10;
    loop {
        if max_loop_count < 0 {
            fail("flow runtime queue is empty!");
            return;
        } else {
            max_loop_count -= 1;
        }
        match get_flow_runtime_status(key) {
            FlowStatus::Starting => {
                // 尝试分析并整理执行队列
                get_flow_runtime_nodes(key).iter().for_each(|node| {
                    if let Some(node_types) = &node.tags {
                        if node_types.iter().any(|t| *t == NodeTag::Logic) {
                            // 逻辑节点本身也要加入进去
                            push_flow_runtime_queue(key, node.clone());
                            set_flow_runtime_status(key, FlowStatus::Queue);
                            println!("logic node added to queue {:?}", get_flow_runtime_queue(key));
                            return;
                        } else {
                            push_flow_runtime_queue(key, node.clone());
                        }
                    }else{
                        push_flow_runtime_queue(key, node.clone());
                    }
                });
            }
            FlowStatus::Queue => {
                // 暂时不实现队列，直接执行
                set_flow_runtime_status(key, FlowStatus::Running);
            }
            FlowStatus::Running => {
                // 执行步骤
                dispatch_step(key)
            }
            FlowStatus::Finished => {
                return;
            }
            FlowStatus::Error => {
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

// 调度执行节点步骤
pub fn dispatch_step(key: &str) {
    let flow = get_flow_runtime(key).unwrap();
    let mut flow_runtime = flow.clone().runtime.unwrap();
    // 决定下一步的走向
    // 根据节点的tags，确定节点需要进行的预操作和后操作
}
