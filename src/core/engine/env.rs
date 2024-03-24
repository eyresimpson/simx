use std::path::Path;
use crate::tools::log::shell::{info, success, warn};

// 环境检查
pub fn check() -> Result<String, String> {
    info("Check Workspace...");
    let config_path = Path::new("config").is_dir();
    let db_path = Path::new("config").is_dir();
    // 检查运行目录下是否有配置文件夹
    if !config_path {
        return Err("Cannot find config path!".parse().unwrap());
    }
    // 检查运行目录下是否有数据库目录（非必须，毕竟现在也没怎么用上）
    if !db_path{
        warn("Cannot find database files, Previous configuration may be lost.")
        // return Err("Cannot find db path!".parse().unwrap());
    }
    
    if !check_python() { 
        warn("Cannot find python in your env， check your configuration.")
    }
    
    success("Check Workspace Done.");
    return Ok("check done.".parse().unwrap())
}

fn check_python() -> bool {
    return true
}