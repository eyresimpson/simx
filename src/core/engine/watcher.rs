use rocket::{build, Config};
use rocket::config::LogLevel;

use crate::conf::simx::get_config;
use crate::core::handler::common::welcome_info;
use crate::core::handler::script::{handle_exec_script, handle_list_script, handle_search_script};
use crate::core::handler::version::{handle_version_current, handle_version_latest, handle_version_list};
use crate::tools::log::shell::info;

pub async fn start_net_watcher() {
    info("Engine Net Services Starting...");
    let conf = get_config();
    // 获取监听地址
    let addr = conf.get("net").unwrap()
        .get("rest-listener-address").unwrap().as_str().unwrap();
    // 获取监听端口
    let port = conf.get("net").unwrap()
        .get("rest-listener-port").unwrap().as_integer().unwrap();
    // 获取工作线程数
    let workers = conf.get("net").unwrap()
        .get("rest-listener-worker").unwrap().as_integer().unwrap();
    // 获取临时文件夹
    let temp_dir = conf.get("net").unwrap()
        .get("rest-listener-temp-path").unwrap().as_str().unwrap();
    let cli_colors = conf.get("engine").unwrap()
        .get("console-log-style").unwrap().as_bool().unwrap();
    // 最大线程，按照引擎最大线程的一半
    let max_thread = conf.get("engine").unwrap()
        .get("max-thread").unwrap().as_integer().unwrap() / 2;
    // http 配置
    let config = Config {
        profile: Default::default(),
        // 绑定的端口
        port: port as u16,
        // 工作线程数
        workers: workers as usize,
        max_blocking: max_thread as usize,
        ident: Default::default(),
        ip_header: None,
        // 监听地址
        address: addr.parse().unwrap(),
        // 缓存目录
        temp_dir: temp_dir.into(),
        keep_alive: 0,
        shutdown: Default::default(),
        log_level: LogLevel::Critical,
        // 是否显示控制台颜色
        cli_colors,
        limits: Default::default(),
        __non_exhaustive: (),
    };
    // 挂载到simx
    // simx中包含所有操作相关内容
    // 此处阻塞了系统的运行，如果后续需要修改，可以去掉 await
    build().configure(config.clone()).mount("/simx", routes![
        // 系统基础信息
        welcome_info,
        handle_version_current,
        handle_version_list,
        handle_version_latest,
        // 脚本相关
        handle_exec_script,
        handle_list_script,
        handle_search_script,
    ]).launch().await.expect("Cannot load rest services.");
}