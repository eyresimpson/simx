use std::path::Path;
use std::process::Command;

use crate::conf::runtime::get_runtime_conf;
use crate::conf::toml::get_env_conf;
use crate::core::common::log::interface::{info, script_fail, script_log, warn};

pub fn exec_python_script(path: &Path) {
    let env_conf = get_env_conf();
    let python = env_conf.get("python");
    if !python.unwrap().get("enable").unwrap().as_bool().unwrap() {
        return;
    }
    if get_runtime_conf("env_python_status").unwrap().eq("not-find") {
        warn("Skip python script exec, cannot find python env.");
        return;
    }
    // 文件名
    let file_name = path.file_name().unwrap().to_str().unwrap_or_default();
    info(format!("Find python in path -> {}", file_name).as_str());
    let exec_path: String;
    // 获取python的执行路径
    exec_path = python.unwrap().get("path").unwrap().as_str().unwrap().parse().unwrap();
    let output = Command::new(exec_path)
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
