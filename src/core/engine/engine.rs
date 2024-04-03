use crate::conf::simx::{get_config, get_net_conf};
use crate::core::db::controller::db_init;
use crate::core::engine::flow::load_and_exec_default_flow;
use crate::core::engine::init::init;
use crate::core::engine::script::load_and_exec_default_script;
use crate::core::engine::watcher::start_net_watcher;
use crate::tools::log::shell::{err, info, success, warn};

/// 引擎核心
pub async fn run() {
    info("Engine Starting...");
    // 尝试加载运行配置
    let conf = get_config();
    let net_conf = get_net_conf();

    // 尝试检查并初始化数据库
    info("System Database checking...");
    if db_init().is_err() {
        err("System Error: Check Your Db Conf!");
    } else {
        success("System database checked successfully.");
    }


    // 执行系统初始化事件
    // 包括环境检查和运行初始化脚本和初始化流
    init();
    // 系统启动完成
    success("Engine has started.");
    // 系统监听
    if net_conf.get("net").unwrap().get("rest-enable-listener").unwrap().as_bool().unwrap() {
        info("Attempt to enable service listening...");
        // 尝试调起网络监听器（阻塞）
        start_net_watcher().await;
    } else {
        warn("Service listening disable, The engine will not be maintained");
    }

    // 运行结束
    success("Engine run out.");
}
