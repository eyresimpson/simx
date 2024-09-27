use std::path::Path;

use crate::core::flow::dispatch::interface::dispatch_flow;
use engine_common::logger::interface::info;

// 这个东西其实就是Json（目前来说），后续可能会进行一些加密的操作
pub async fn exec_fl_flow(path: &Path) {
    dispatch_flow(path).await;
}

// 执行toml流
pub fn exec_toml_flow(path: &Path) {
    info(path.to_str().unwrap())
}

// 执行xml流
pub fn exec_xml_flow(path: &Path) {
    info(path.to_str().unwrap())
}