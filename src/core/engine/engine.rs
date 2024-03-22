use crate::conf::simx::get_config;
use crate::core::db::controller::db_init;
use crate::core::engine::env::check;
use crate::core::engine::flow::load_and_exec_default_flow;
use crate::core::engine::script::load_and_exec_default_script;
use crate::core::engine::watcher::start_net_watcher;
use crate::tools::log::shell::{err, info, success, warn};

// 引擎核心
pub fn run() {
    info("Engine Starting...");
    // 检查工作环境（当前目录）
    match check() {
        Ok(..) => warn("Cannot success run check with local"),
        _ => err("Check runtime env failed, check your env!")
    }
    // 尝试加载运行配置
    let conf = get_config();

    info("System Database Init...");
    // 尝试初始化数据库
    // 其实目前数据库可有可无，系统复杂度还不至于全部走数据库，后续流程上了再考虑深入
    if db_init().is_err() {
        err("System Error: ");
        err("Check Your Db Conf!");
    }

    success("System Database Load succeed.");
    success("Engine has started.");

    // 默认脚本
    if conf.get("engine").unwrap().get("run-default-script").unwrap().as_bool().unwrap() {
        // 尝试加载默认脚本
        info("Default script running...");
        load_and_exec_default_script();
        success("Run default script done.");
    } else {
        info("Skip default script running.");
    }

    // 默认流
    if conf.get("engine").unwrap().get("run-default-flow").unwrap().as_bool().unwrap() {
        // 尝试加载默认流
        info("Default flow running...");
        load_and_exec_default_flow();
        success("Run default flow done.");
    } else {
        info("Skip default flow running.");
    }

    // 网络监听
    if conf.get("net").unwrap().get("http-enable-listener").unwrap().as_bool().unwrap() {
        info("Attempt to enable service listening...");
        // 尝试调起网络监听器（同步阻塞）
        start_net_watcher();
    } else {
        warn("Service listening disable, The engine will not be maintained");
    }


    // 运行结束
    success("Engine run out.");
}
