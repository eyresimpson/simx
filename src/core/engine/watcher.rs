use rocket::{build, Config};
use rocket::config::LogLevel;

use crate::conf::simx::get_config;
use crate::core::handler::script::{handle_exec_script, handle_list_script, handle_search_script};
use crate::core::handler::version::{handle_version_current, handle_version_latest, handle_version_list};
use crate::tools::log::shell::info;

pub async fn start_net_watcher() {
    info("Engine Net Services Starting...");
    let conf = get_config();
    // 获取监听地址
    let addr = conf.get("net").unwrap().get("http-listener-address").unwrap().as_str().unwrap();
    // 获取监听端口
    let port = conf.get("net").unwrap().get("http-listener-port").unwrap().as_integer().unwrap();
    let workers = conf.get("net").unwrap().get("http-listener-worker").unwrap().as_integer().unwrap();
    let tmp_path = conf.get("net").unwrap().get("http-listener-temp-path").unwrap().as_str().unwrap();
    let config = Config {
        profile: Default::default(),
        port: port as u16,
        workers: workers as usize,
        max_blocking: 0,
        ident: Default::default(),
        ip_header: None,
        address: addr.parse().unwrap(),
        temp_dir: tmp_path.into(),
        keep_alive: 0,
        shutdown: Default::default(),
        log_level: LogLevel::Critical,
        cli_colors: false,
        limits: Default::default(),
        __non_exhaustive: (),
    };
    build().configure(config).mount("/simx", routes![
        // 脚本相关
        handle_exec_script,
        handle_list_script,
        handle_search_script,
        // 版本相关
        handle_version_current,
        handle_version_list,
        handle_version_latest,
    ]).launch().await.expect("Cannot Handle Rest Services");
    // start_http_watcher();
}

