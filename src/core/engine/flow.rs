use std::path::Path;

use crate::tools::log::shell::info;

// 流引擎核心
pub fn exec_flow() {}

pub fn load_and_exec_default_flow() {
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new("init").join("flow").is_dir() {
        // 遍历文件夹下的所有内容
        exec_flow();
    } else {
        info("No init flow found, Skip...")
    }
}
