use crate::handler::serve::http::handler::common::welcome_info;
use crate::handler::serve::http::handler::script::handle_search_script;
use crate::handler::serve::http::handler::status::{handle_version_current, handle_version_latest, handle_version_list};
use rocket::config::LogLevel;
use rocket::{build, routes, Config};

pub async fn start_net_watcher() {


    // 获取监听地址
    let addr = "127.0.0.1";
    // 获取监听端口
    let port = 9898;
    // 获取工作线程数
    let workers = 24;
    // 获取临时文件夹
    let temp_dir = "tmp";
    let cli_colors = true;
    // 最大线程，按照引擎最大线程的一半
    let max_blocking = 32;
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
        // log_level: LogLevel::Critical,
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
        // handle_list_script,
        handle_search_script,
        // 流程相关
        // handle_list_flow,
        // handle_search_flow,
        // handle_exec_flow_by_path
    ]).launch().await.expect("Cannot load rest services.");
}