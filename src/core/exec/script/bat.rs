use std::env;
use std::path::Path;
use std::process::Command;

use crate::conf::simx::get_config;
use crate::tools::log::shell::{info, script_err, script_log, warn};

pub fn exec_bat_script(path: &Path) {
    info(format!("Find Shell in path -> {:?}", path).as_str());
    let conf = get_config();
    // 获取操作系统类型
    let os = env::consts::OS;
    if os == "windows"{
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
