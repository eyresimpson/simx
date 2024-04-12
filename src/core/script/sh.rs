use std::env;
use std::path::Path;
use std::process::Command;

use crate::conf::toml::get_env_conf;
use crate::core::common::log::shell::{info, script_err, script_log, warn};

pub fn exec_shell_script(path: &Path) {
    info(format!("Find Shell in path -> {:?}", path).as_str());
    let conf = get_env_conf();
    // 获取操作系统类型
    let os = env::consts::OS;
    if os == "linux" || os == "macos" {
        if os == "macos" {
            if conf.get("shell").unwrap().get("enable-macos").unwrap().as_bool().unwrap() {
                warn("Run sh script on mac!")
            } else {
                info("Skip executing the sh script on mac.");
                return;
            }
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
    }else {
        info("Incompatible operating system, skip")
    }
}
