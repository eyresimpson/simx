use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref RUNTIME_ENGINE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn set_engine_info(key: &str, value: &str) {
    let mut data = RUNTIME_ENGINE.lock().unwrap();
    data.insert(key.to_string(), value.to_string());
}

// 修改指定运行时配置
pub fn set_runtime_conf(key: &str, value: &str) {
    let mut data = RUNTIME_ENGINE.lock().unwrap();
    data.insert(key.to_string(), value.to_string());
}