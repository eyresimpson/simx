use crate::core::flow::exec::node::exec_node;
use crate::entity::flow::{Flow, FlowData, Node};
use crate::tools::log::interface::{info, success};

// 执行标准化的流
pub async fn exec_standardisation_flow(flow: Flow) {
    info(format!("flow {{ {} }} will be exec.", flow.flow_name).as_str());
    // 流数据对象（全部节点都可以对其进行修改，在流程的整个生命周期都可用）
    // let mut data: Data = Data { data: Default::default() };
    let data: FlowData = FlowData {
        basics: Default::default(),
        params: Default::default(),
        data: Default::default(),
    };

    // 执行流
    exec_steps(flow.steps, data).await;

    success(format!("flow {{ {} }} has be exec success.", flow.flow_name).as_str());
}

// 执行节点列表
// 拆分出来的目的是为了便于子节点的调用
pub async fn exec_steps(steps: Vec<Node>, mut flow_data: FlowData) {
    // 计数器，计量当前在第几个节点
    let mut i = 0;

    // 流程节点执行
    for mut node in steps {
        i = i + 1;
        // 当前节点索引
        node.attr.insert("node_index".parse().unwrap(), i.to_string());
        // let mut data: FlowData = flow_data.clone();
        // 将执行的结果保存到流对象中
        exec_node(node, &mut flow_data).await;
    }
}