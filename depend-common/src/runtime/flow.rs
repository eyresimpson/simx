use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use crate::entity::flow::{Blueprint, Flow, FlowData, FlowStatus, Node};
use crate::entity::simx::SimxFlow;
use crate::logger::interface::warn;
use lazy_static::lazy_static;

lazy_static! {
    // 流信息集合
    static ref FLOW_INFO: Mutex<HashMap<String, SimxFlow>> = Mutex::new(HashMap::new());
    // 运行时集合
    static ref RUNTIME_FLOW: Mutex<HashMap<String, Flow>> = Mutex::new(HashMap::new());
}

// 设置流信息，key是路径，value是流信息
pub fn set_flow_info(key: &str, value: SimxFlow) {
    let mut data = FLOW_INFO.lock().unwrap();
    data.insert(key.to_string(), value);
}

// 设置流运行时
pub fn set_flow_runtime(key: &str, value: Flow) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    data.insert(key.to_string(), value);
}

// 获取流运行时
pub fn get_flow_runtime(key: &str) -> Option<Flow> {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.get(key).cloned()
}

// 删除指定流的运行时
pub fn del_flow_runtime(key: &str) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    data.remove(key);
}

// 设置流当前状态
pub fn set_flow_runtime_status(key: &str, flow_status: FlowStatus) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    if let Some(flow) = data.get_mut(key) {
        if let Some(ref mut runtime) = flow.runtime {
            runtime.status = flow_status;
        } else {
            warn(format!("flow {} runtime status is uninitialized!", key).as_str());
        }
    } else {
        warn(format!("flow {} runtime cannot find.", key).as_str())
    }
}

// 获取流当前状态
pub fn get_flow_runtime_status(key: &str) -> FlowStatus {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.get(key).cloned().unwrap().runtime.unwrap().status
}

// 获取流节点列表
pub fn get_flow_runtime_nodes(key: &str) -> Vec<Node> {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.get(key).cloned().unwrap().nodes
}

pub fn get_flow_runtime_node_by_id(key: &str, node_id: &str) -> Option<Node> {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.get(key).cloned().unwrap().nodes.iter().find(|node| node.id == node_id).cloned()
}

pub fn get_flow_runtime_flow_data(key: &str) -> FlowData {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.get(key).cloned().unwrap().runtime.unwrap().data
}

pub fn set_flow_runtime_flow_data(key: &str, data: FlowData) {
    let mut runtime = RUNTIME_FLOW.lock().unwrap();
    runtime.get_mut(key).unwrap().runtime.as_mut().unwrap().data = data;
}