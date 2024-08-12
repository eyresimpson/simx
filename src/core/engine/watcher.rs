use rocket::{build, Config};
use rocket::config::LogLevel;

use crate::conf::toml::{get_engine_config, get_net_conf};
use crate::core::common::log::interface::{info, warn};
use crate::net::http::handler::common::welcome_info;
use crate::net::http::handler::flow::{handle_exec_flow_by_path, handle_list_flow, handle_search_flow};
use crate::net::http::handler::script::{handle_exec_script, handle_list_script, handle_search_script};
use crate::net::http::handler::status::{handle_version_current, handle_version_latest, handle_version_list};

pub async fn start_net_watcher() {
    let net_conf = get_net_conf();

    // 系统监听
    if !net_conf.get("net").unwrap().get("rest-enable-listener").unwrap().as_bool().unwrap() {
        warn("Service listening disable, The engine will not be maintained");
        return;
    }
    let net_conf = get_net_conf();
    let engine_conf = get_engine_config();
    // 获取监听地址
    let addr = net_conf.get("net").unwrap()
        .get("rest-listener-address").unwrap().as_str().unwrap();
    // 获取监听端口
    let port = net_conf.get("net").unwrap()
        .get("rest-listener-port").unwrap().as_integer().unwrap() as u16;
    // 获取工作线程数
    let workers = net_conf.get("net").unwrap()
        .get("rest-listener-worker").unwrap().as_integer().unwrap() as usize;
    // 获取临时文件夹
    let temp_dir = net_conf.get("net").unwrap()
        .get("rest-listener-temp-path").unwrap().as_str().unwrap();
    let cli_colors = engine_conf.get("engine").unwrap()
        .get("console-log-style").unwrap().as_bool().unwrap();
    // 最大线程，按照引擎最大线程的一半
    let max_blocking = (engine_conf.get("engine").unwrap()
        .get("max-thread").unwrap().as_integer().unwrap() / 2) as usize;
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
        handle_exec_script,
        handle_list_script,
        handle_search_script,
        // 流程相关
        handle_list_flow,
        handle_search_flow,
        handle_exec_flow_by_path
    ]).launch().await.expect("Cannot load rest services.");
}