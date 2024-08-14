use rocket::{build, Config};
use rocket::config::LogLevel;

use crate::core::runtime::config::get_simx_config;
use crate::net::http::handler::common::welcome_info;
use crate::net::http::handler::flow::{handle_exec_flow_by_path, handle_list_flow, handle_search_flow};
use crate::net::http::handler::script::{handle_list_script, handle_search_script};
use crate::net::http::handler::status::{handle_version_current, handle_version_latest, handle_version_list};
use crate::tools::log::interface::{info, warn};

pub async fn start_net_watcher() {
    let net_config = get_simx_config().net;

    // 系统监听
    if !net_config.rest_enable_listener {
        warn("Service listening disable, The engine will not be maintained");
        return;
    }
    let engine_conf = get_simx_config().engine;
    // 获取监听地址
    let addr = net_config.rest_listener_address;
    // 获取监听端口
    let port = net_config.rest_listener_port;
    // 获取工作线程数
    let workers = net_config.rest_listener_worker;
    // 获取临时文件夹
    let temp_dir = net_config.rest_listener_temp_path;
    let cli_colors = engine_conf.console_log_style;
    // 最大线程，按照引擎最大线程的一半
    let max_blocking = (engine_conf.max_thread / 2) as usize;
    // http 配置
    let config = Config {
        profile: Default::default(),
        // 不打印Rocket的日志
        log_level: LogLevel::Off,
        // 绑定的端口
        port,
        // 工作线程数
        workers,
        // 最大线程
        max_blocking,
        ident: Default::default(),
        ip_header: None,
        // 监听地址
        address: addr.parse().unwrap(),
        // 缓存目录
        temp_dir: temp_dir.into(),
        keep_alive: 0,
        shutdown: Default::default(),
        // 是否显示控制台颜色
        cli_colors,
        limits: Default::default(),
        __non_exhaustive: (),
    };
    info(format!("Engine Http Services has started in {}:{}.", addr, port).as_str());
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
        handle_list_script,
        handle_search_script,
        // 流程相关
        handle_list_flow,
        handle_search_flow,
        handle_exec_flow_by_path
    ]).launch().await.expect("Cannot load rest services.");
}