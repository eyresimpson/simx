use std::path::Path;
use std::process::Command;

use crate::conf::runtime::get_runtime_conf;
use crate::core::common::log::interface::{info, script_fail, script_log, warn};
use crate::entity::config::engine::get_engine_config;

pub fn exec_python_script(path: &Path) {
    let env_config = get_engine_config().env;
    let python = env_config.python_path;
    if !env_config.enable_python_script {
        return;
    }
    if get_runtime_conf("env_python_status").unwrap().eq("not-find") {
        warn("Skip python script exec, cannot find python env.");
        return;
    }
    // 文件名
    let file_name = path.file_name().unwrap().to_str().unwrap_or_default();
    info(format!("Find python in path -> {}", file_name).as_str());
    // 获取python的执行路径
    let output = Command::new(python)
        .arg(path)
        .output()
        .expect("Failed to execute py script");
    // 输出执行结果
    if &*String::from_utf8_lossy(&output.stdout) != "" {
        script_log(&*String::from_utf8_lossy(&output.stdout).trim());
    }
    if &*String::from_utf8_lossy(&output.stderr) != "" {
        script_fail(&*String::from_utf8_lossy(&output.stderr).trim());
    }
}
