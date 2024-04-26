use std::env;
use std::path::Path;
use std::process::Command;

use crate::core::common::log::interface::{info, script_fail, script_log};

pub fn exec_powershell_script(path: &Path){
    info(format!("Find powershell in path -> {:?}", path).as_str());
    // 获取操作系统类型
    let os = env::consts::OS;
    // 仅windows 执行
    if os == "windows" {
        let output = Command::new("powershell")
            .arg("-File")
            .arg(path)
            .output()
            .expect("Failed to execute script");
        // 输出执行结果
        if String::from_utf8_lossy(&output.stdout) != "" {
            script_log(String::from_utf8_lossy(&output.stdout).trim());
        }
        if String::from_utf8_lossy(&output.stderr) != "" {
            script_fail(String::from_utf8_lossy(&output.stderr).trim());
        }
    }else {
        info("Incompatible operating system, skip")
    }
}
