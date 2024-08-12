use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    // 引擎基础运行时
    static ref ENGINE_RUNTIME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // 全局配置
    static ref CONFIG_RUNTIME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // 流运行时
    static ref FLOW_RUNTIME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // 脚本运行时
    static ref SCRIPT_RUNTIME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    // 扩展运行时
    static ref EXT_RUNTIME: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 设置引擎状态
pub fn set_runtime_engine_info(engine: HashMap<String, String>) {
    let mut data = ENGINE_RUNTIME.lock().unwrap();
    data.clear();
    data.extend(engine);
}

// 获取引擎状态
pub fn get_runtime_engine_info() -> Option<HashMap<String, String>> {
    let data = ENGINE_RUNTIME.lock().unwrap();
    return Some(data.clone());
}

// 设置当前流列表
pub fn set_runtime_flow_list() {}

// 获取当前流列表
pub fn get_runtime_flow_list() -> Option<Vec<String>> {
    let data = FLOW_RUNTIME.lock().unwrap();
    data.iter().for_each(|(k, v)| {
        println!("{} = {}", k, v);
    });
    return Some(vec![]);
}

// 重新加载流列表
pub fn reload_runtime_flow_list() {}


// 设置当前脚本列表
pub fn set_runtime_script_list() {}

// 重新加载脚本列表
pub fn reload_runtime_script_list() {}

// 设置当前插件列表
pub fn set_runtime_ext_list() {}

// 重新加载插件列表
pub fn reload_runtime_ext_list() {}

// 重新加载配置
pub fn reload_runtime_config() {}
//

//
// pub fn get_runtime_engine_map(key: &str) -> Option<String> {
//     let data = ENGINE_RUNTIME.lock().unwrap();
//     data.get(key).cloned()
// }
//
// // 获取指定运行时配置
// pub fn get_runtime_flow_map(key: &str) -> Option<String> {
//     let data = FLOW_RUNTIME.lock().unwrap();
//     data.get(key).cloned()
// }
//
// pub fn get_runtime_script_map(key: &str) -> Option<String> {
//     let data = SCRIPT_RUNTIME.lock().unwrap();
//     data.get(key).cloned()
// }
//
// pub fn get_runtime_ext_map(key: &str) -> Option<String> {
//     let data = EXT_RUNTIME.lock().unwrap();
//     data.get(key).cloned()
// }
//
//
// // 修改指定运行时配置
// pub fn set_runtime_flow_map(key: &str, value: &str) {
//     let mut data = FLOW_RUNTIME.lock().unwrap();
//     data.insert(key.to_string(), value.to_string());
// }
//
// // 修改指定运行时配置
// pub fn set_runtime_script_map(key: &str, value: &str) {
//     let mut data = SCRIPT_RUNTIME.lock().unwrap();
//     data.insert(key.to_string(), value.to_string());
// }
//
// // 修改指定运行时配置
// pub fn set_runtime_ext_map(key: &str, value: &str) {
//     let mut data = EXT_RUNTIME.lock().unwrap();
//     data.insert(key.to_string(), value.to_string());
// }
//
// // 修改指定运行时配置
// pub fn set_runtime_engine_map(key: &str, value: &str) {
//     let mut data = ENGINE_RUNTIME.lock().unwrap();
//     data.insert(key.to_string(), value.to_string());
// }
