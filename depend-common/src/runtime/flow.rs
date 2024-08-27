use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::entity::simx::SimxFlow;

lazy_static! {
    static ref RUNTIME_FLOW: Mutex<HashMap<String, SimxFlow>> = Mutex::new(HashMap::new());
}

// 设置流信息，key是路径，value是流信息
pub fn set_flow_info(key: &str, value: SimxFlow) {
    let mut data = RUNTIME_FLOW.lock().unwrap();
    data.insert(key.to_string(), value);
}