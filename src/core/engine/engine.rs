use crate::conf::simx::get_config;
use crate::core::db::controller::db_init;
use crate::core::engine::workspace::check;
use crate::core::engine::flow::load_and_exec_default_flow;
use crate::core::engine::script::load_and_exec_default_script;
use crate::core::engine::watcher::start_net_watcher;
use crate::tools::log::shell::{err, info, success, warn};

/// 引擎核心
pub async fn run() {
    info("Engine Starting...");

    // 检查工作环境（当前目录）
    if let Err(_) = check() {
        err("Check runtime env failed, Engine Shutdown.");
        return;
    } else {
        success("Workspace check done.");
    }

    // 尝试加载运行配置
    let conf = get_config();

    // 尝试初始化数据库
    info("System Database Init...");
    if db_init().is_err() {
        err("System Error: Check Your Db Conf!");
    } else {
        success("System Database Load succeed.");
    }

    success("Engine has started.");

    // 初始化脚本
    if conf.get("engine").unwrap().get("run-init-script").unwrap().as_bool().unwrap() {
        info("Default script running...");
        load_and_exec_default_script();
        success("Run init script done.");
    } else {
        info("Skip init script running.");
    }

    // 初始流
    if conf.get("engine").unwrap().get("run-init-flow").unwrap().as_bool().unwrap() {
        info("Default flow running...");
        load_and_exec_default_flow();
        success("Run init flow done.");
    } else {
        info("Skip init flow running.");
    }

    // 系统监听
    if conf.get("net").unwrap().get("rest-enable-listener").unwrap().as_bool().unwrap() {
        info("Attempt to enable service listening...");
        // 尝试调起网络监听器（阻塞）
        start_net_watcher().await;
    } else {
        warn("Service listening disable, The engine will not be maintained");
    }

    // 运行结束
    success("Engine run out.");
}
