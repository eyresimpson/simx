use std::fs;
use std::path::Path;

use crate::conf::runtime::set_runtime_conf;
use crate::conf::toml::get_env_conf;
use crate::core::common::log::interface::{fail, info, success, warn};
use crate::core::env::python::check_python;

/// 环境检查
pub fn env_check() -> Result<String, String> {
    // 尝试加载运行配置
    let env_conf = get_env_conf();

    // 检查运行目录下是否有配置文件夹
    // 这个配置是必需的，一旦找不到，立即报错退出
    info("Check Workspace...");
    let config_path = Path::new("conf").is_dir();
    if !config_path {
        fail("Cannot find simx main config path, please check simx workspace.");
        return Err("Cannot find config path!".to_string());
    }

    // 检查运行目录下是否有数据库目录（非必须，毕竟现在也没怎么用上）
    let db_path = Path::new("db").is_dir();
    if !db_path {
        // 如果数据库文件不存在，系统会主动初始化一个新的，因此不算致命错误
        warn("Cannot find database files, Previous configuration may be lost.");
        fs::create_dir("db").expect("Engine cannot fix workspace, Please check your env.");
    }

    // 检查运行目录下是否有缓存目录
    let db_path = Path::new("tmp").is_dir();
    if !db_path {
        info("Cannot find tmp dir, system will automatically rebuild this directory.");
        // 重建缓存目录
        // 这个目录下会保存临时内容，包括引擎运行时产生的临时数据和Rest服务运行时的缓存文件
        fs::create_dir("tmp").expect("Engine cannot fix workspace, Please check your env.");
    }

    

    // 检查是否有 Python 环境
    if !check_python(env_conf.clone()) {
        warn("Cannot find python in your env, check your configuration.");
        set_runtime_conf("env_python_status", "not-find");
    } else {
        set_runtime_conf("env_python_status", "python3")
    }

    success("Check Workspace Done.");
    Ok("check done.".to_string())
}

