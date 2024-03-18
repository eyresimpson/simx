use std::env;
use std::path::Path;
use std::process::Command;

use crate::tools::log::shell::{script_err, script_log, warn};

pub fn exec_shell_script(path: &Path) {
    // 获取操作系统类型
    let os = env::consts::OS;
    if os == "linux" || os == "macos" {
        if os == "macos" {
            warn("As an unexpected result, execute sh script in macos, which may not have been designed for this system")
        }
        let output = Command::new("sh")
            .arg(path)
            .output()
            .expect("Failed to execute script");
        // 输出执行结果
        if &*String::from_utf8_lossy(&output.stdout) != "" {
            script_log(&*String::from_utf8_lossy(&output.stdout).trim());
        }
        if &*String::from_utf8_lossy(&output.stderr) != "" {
            script_err(&*String::from_utf8_lossy(&output.stderr).trim());
        }
    }
}
