use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::core::flow::entity::standardisation::Flow;

pub fn resolver_flow(path: &Path) -> Flow {
    // 尝试读取流文件
    let mut file = File::open(path).unwrap();
    let mut flow_str = String::new();
    file.read_to_string(&mut flow_str).unwrap();
    // 尝试解析流文件为统一流程对象
    let flow: Flow = serde_json::from_str(&flow_str).unwrap();
    // err(flow.clone().create_date.as_str());
    return flow;
}

// 将Flow编码为flow文件
pub fn encoding_flow_to_file(flow: Flow, path: String) {
    let serialized_flow = serde_json::to_string(&flow).unwrap();

    // 将 JSON 字符串写入文件
    let mut file = File::create(path).unwrap();
    file.write_all(serialized_flow.as_bytes()).unwrap();
}