use std::path::Path;
use engine_common::entity::flow::{Flow, FlowData, FlowRuntimeModel, FlowStatus, Node};
use engine_common::runtime::flow::set_flow_runtime;
use crate::core::flow::resolver::interface::flow_resolver;

// 调度执行流
// 此方法会根据流文件的path或json，生成Flow运行时并调度执行
// 第一个参数是流文件路径，第二个是流文件内容，两个参数必须有一个不为空
// 建议在新线程中执行此方法
pub fn dispatch_flow(path: &Path, content: String){
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
    dispatch_node(flow);

}

// 调度执行节点
fn dispatch_node(flow: Flow){

}

// 调度执行节点步骤
pub fn dispatch_step(){

}