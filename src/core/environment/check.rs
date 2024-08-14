use std::fs;
use std::path::Path;

use crate::conf::runtime::set_runtime_conf;
use crate::core::environment::python::check_python;
use crate::core::runtime::config::get_simx_config;
use crate::tools::log::interface::{fail, info, success, warn};

/// 环境检查
pub fn env_check() -> Result<String, String> {
    let env_config = get_simx_config().env;
    // 检查运行目录下是否有配置文件夹
    // 这个配置是必需的，一旦找不到，立即报错退出
    info("Check Workspace...");
    let config_path = Path::new("conf").is_dir();
    if !config_path {
        fail("Cannot find simx main config path, please check simx workspace.");
        return Err("Cannot find config path!".to_string());
    }

    // 检查运行目录下是否有缓存目录
    let tmp_path = Path::new("tmp").is_dir();
    if !tmp_path {
        info("Cannot find tmp dir, system will automatically rebuild this directory.");
        // 重建缓存目录
        // 这个目录下会保存临时内容，包括引擎运行时产生的临时数据和Rest服务运行时的缓存文件
        fs::create_dir("tmp").expect("Engine cannot fix workspace, Please check your environment.");
    }


    // 检查是否有 Python 环境
    if !check_python(env_config.python_path) {
        warn("Cannot find python in your environment, check your configuration.");
        set_runtime_conf("env_python_status", "not-find");
    } else {
        set_runtime_conf("env_python_status", "python3")
    }

    success("Check Workspace Done.");
    Ok("check done.".to_string())
}