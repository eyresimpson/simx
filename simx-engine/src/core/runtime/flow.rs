use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

use simx_common::entity::simx::SimxFlow;

lazy_static! {
    static ref RUNTIME_FLOW: Mutex<HashMap<String, SimxFlow>> = Mutex::new(HashMap::new());
}

// 设置流信息，key是路径，value是流信息
pub fn set_flow_info(key: &str, value: SimxFlow) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    data.insert(key.to_string(), value);
}

// 获取流信息，key是路径，返回流信息
pub fn get_flow_info(key: &str) -> Option<SimxFlow> {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.get(key).cloned()
}

// 获取所有流的信息，以数组的形式
pub fn get_all_flow_info() -> Vec<SimxFlow> {
    let data = RUNTIME_FLOW.lock().unwrap();
    data.values().cloned().collect()
}