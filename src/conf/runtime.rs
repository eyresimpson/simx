use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref RUNTIME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 获取指定运行时配置
pub fn get_runtime_conf(key: &str) -> Option<String> {
    let data = RUNTIME.lock().unwrap();
    data.get(key).cloned()
}

// 修改指定运行时配置
pub fn set_runtime_conf(key: &str, value: &str) {
    let mut data = RUNTIME.lock().unwrap();
    data.insert(key.to_string(), value.to_string());
}

// 删除指定运行时配置
// pub fn del_runtime_conf(key: &str) -> Option<String> {
//     let mut data = RUNTIME.lock().unwrap();
//     data.remove(key)
// }
