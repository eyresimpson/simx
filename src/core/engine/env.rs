use std::path::Path;
use toml::Value;
use crate::conf::simx::get_config;
use crate::tools::log::shell::{info, success, warn};

/// 环境检查
pub fn check() -> Result<String, String> {
    // 尝试加载运行配置
    let conf = get_config();
    info("Check Workspace...");

    // 检查运行目录下是否有配置文件夹
    let config_path = Path::new("config").is_dir();
    if !config_path {
        return Err("Cannot find config path!".to_string());
    }

    // 检查运行目录下是否有数据库目录（非必须，毕竟现在也没怎么用上）
    let db_path = Path::new("config").is_dir();
    if !db_path {
        warn("Cannot find database files, Previous configuration may be lost.");
    }

    // 检查是否有 Python 环境
    if !check_python(conf.clone()) {
        warn("Cannot find python in your env, check your configuration.");
    }

    success("Check Workspace Done.");
    Ok("check done.".to_string())
}

fn check_python(conf: Value) -> bool {
    let current_python  = conf.get("python").unwrap().get("path").unwrap().as_str().unwrap();
    // 注意！ Windows 10/11即使没有安装，也不会报错（因为微软不知道怎么想得，如果未安装居然会默认打开应用商店...）
    std::process::Command::new(current_python).arg("--version").output().is_ok()
}
