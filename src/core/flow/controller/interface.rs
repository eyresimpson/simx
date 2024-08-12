use std::path::Path;

use crate::core::common::log::interface::info;
use crate::core::flow::controller::check::check_require;
use crate::core::flow::exec::flow::exec_standardisation_flow;
use crate::core::flow::resolver::interface::flow_resolver;

// 这个东西其实就是Json（目前来说），后续可能会进行一些加密的操作
pub async fn exec_fl_flow(path: &Path) {
    // 解析文件为标准流
    let flow = flow_resolver(path);
    // 检查运行要求
    if !check_require(flow.clone()) { return; }
    // 执行流
    exec_standardisation_flow(flow).await;
    // info(path.to_str().unwrap())
}

// 执行toml流
pub fn exec_toml_flow(path: &Path) {
    info(path.to_str().unwrap())
}

// 执行json流
// pub fn exec_json_flow(path: &Path) {
//     info(path.to_str().unwrap())
// }

// 执行xml流
pub fn exec_xml_flow(path: &Path) {
    info(path.to_str().unwrap())
}