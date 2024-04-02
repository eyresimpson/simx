use std::collections::HashMap;

use crate::core::flow::entity::standardisation::Flow;
use crate::core::flow::etalon::dispatch::dispatch;
use crate::tools::log::shell::{info, success};

// 执行标准化的流
pub fn exec_standardisation_flow(flow: Flow) {
    info(format!("flow {{ {} }} will be exec.", flow.flow_name).as_str());
    // 流数据对象（全部节点都可以对其进行修改，在流程的整个生命周期都可用）
    let mut data: HashMap<String, String> = Default::default();
    data.insert("flow_step".parse().unwrap(), "0".parse().unwrap());
    // 流程节点执行
    for step in flow.steps {
        // 将执行的结果保存到流对象中
        data = dispatch(step.handler,step.mold, data, step.data);
    }
    success(format!("flow {{ {} }} has be exec success.", flow.flow_name).as_str());
}