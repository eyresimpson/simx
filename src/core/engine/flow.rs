use std::path::Path;

use crate::tools::log::shell::warn;

// 流引擎核心
fn load_flow() {}

pub fn exec_flow() {
    load_flow()
}

pub fn load_and_exec_default_flow() {
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new("./flow/").is_dir() {
        // 遍历文件夹下的所有内容
        exec_flow();
    } else {
        warn("No default flow found, Skip...")
    }
    exec_flow();
}