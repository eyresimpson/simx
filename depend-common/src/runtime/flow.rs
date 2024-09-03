use std::collections::HashMap;
use std::sync::Mutex;

use crate::entity::flow::{Flow, FlowStatus, Node};
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
pub fn set_flow_runtime_status(key: &str, flowStatus: FlowStatus) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    if let Some(flow) = data.get_mut(key) {
        if let Some(ref mut runtime) = flow.runtime {
            runtime.status = flowStatus;
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

// 压入执行队列（尾部添加）
pub fn push_flow_runtime_queue(key: &str, node: Node) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    if let Some(flow) = data.get_mut(key) {
        if let Some(ref mut runtime) = flow.runtime {
            runtime.queue.push_back(node);
        } else {
            warn(format!("flow {} runtime status is uninitialized!", key).as_str());
        }
    } else {
        warn(format!("flow {} runtime cannot find.", key).as_str())
    }
}
// 压出执行队列（获取首位）
pub fn pull_flow_runtime_queue(key: &str) -> Option<Node> {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    if let Some(flow) = data.get_mut(key) {
        if let Some(ref mut runtime) = flow.runtime {
            runtime.queue.pop_front()
        } else {
            warn(format!("flow {} runtime status is uninitialized!", key).as_str());
            None
        }
    } else {
        warn(format!("flow {} runtime cannot find.", key).as_str());
        None
    }
}
// 清空执行队列
pub fn clean_flow_runtime_queue(key: &str) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    if let Some(flow) = data.get_mut(key) {
        if let Some(ref mut runtime) = flow.runtime {
            runtime.queue.clear()
        } else {
            warn(format!("flow {} runtime status is uninitialized!", key).as_str());
        }
    } else {
        warn(format!("flow {} runtime cannot find.", key).as_str())
    }
}