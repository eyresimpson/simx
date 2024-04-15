use std::collections::HashMap;

use crate::core::common::log::shell::{info, success};
use crate::core::flow::entity::standardisation::Flow;
use crate::core::flow::etalon::dispatch::dispatch;

// 执行标准化的流
pub fn exec_standardisation_flow(flow: Flow) {
    info(format!("flow {{ {} }} will be exec.", flow.flow_name).as_str());
    // 流数据对象（全部节点都可以对其进行修改，在流程的整个生命周期都可用）
    let mut data: HashMap<String, String> = Default::default();
    // 计数器，计量当前在第几个节点
    let mut i = 0;
    // 流程节点执行
    for mut node in flow.nodes {
        i = i + 1;
        // TODO： 这个后续也许应该加一个runtime的对象记录，而不是全都放在这里
        node.attr.insert("node_index".parse().unwrap(), i.to_string());
        // 将执行的结果保存到流对象中
        data = dispatch(node.handler, node.mold, data, node.attr, node.steps);
    }
    success(format!("flow {{ {} }} has be exec success.", flow.flow_name).as_str());
}