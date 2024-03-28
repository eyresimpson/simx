use std::path::Path;
use crate::tools::log::shell::info;

pub fn exec_fl_flow(path: &Path) {
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