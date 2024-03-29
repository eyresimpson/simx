use std::path::Path;
use std::process::Command;

use crate::conf::simx::get_config;
use crate::tools::log::shell::{info, script_err, script_log};

pub fn exec_python_script(path: &Path) {
    let conf = get_config();
    let python = conf.get("python");
    if !python.unwrap().get("enable").unwrap().as_bool().unwrap() {
        return;
    }
    let file_name = path.file_name().unwrap().to_str().unwrap_or_default();
    info(format!("Find python in path -> {}", file_name).as_str());
    let exec_path: String;
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
        script_err(&*String::from_utf8_lossy(&output.stderr).trim());
    }
}
