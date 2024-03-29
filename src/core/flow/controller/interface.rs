use std::path::Path;
use crate::core::flow::etalon::etalon::exec_standardisation_flow;
use crate::core::flow::resolver::flow::resolver_flow;
use crate::tools::log::shell::info;

pub fn exec_fl_flow(path: &Path) {
    // 解析文件为标准流
    let flow = resolver_flow(path);
    // 执行流
    exec_standardisation_flow(flow);
    info(path.to_str().unwrap())
}

pub fn exec_toml_flow(path: &Path) {
    info(path.to_str().unwrap())
}

pub fn exec_json_flow(path: &Path) {
    info(path.to_str().unwrap())
}

pub fn exec_xml_flow(path: &Path) {
    info(path.to_str().unwrap())
}