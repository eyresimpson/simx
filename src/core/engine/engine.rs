// 引擎核心
use crate::core::engine::env::{check, load_configuration};
use crate::core::engine::flow::load_and_exec_default_flow;
use crate::core::engine::script::load_and_exec_default_script;
use crate::core::engine::watcher::{start_cron_watcher, start_net_watcher};
use crate::tools::log::shell::{err, info, success, warn};

// 运行引擎
pub fn run() {
    info("Engine Starting...");

    // 检查工作环境（当前目录）
    if !check() {
        warn("Cannot success run check with local");
        err("Check runtime env failed, check your env!");
    }
    // 尝试加载运行配置
    load_configuration();

    start_cron_watcher();
    // 调用流服务

    // 尝试加载默认脚本
    info("Default script running...");
    load_and_exec_default_script();
    success("Run default script done.");

    // 尝试加载默认流
    info("Default flow running...");
    load_and_exec_default_flow();
    success("Run default flow done.");

    success("Engine has started..");

    info("Attempt to enable service listening...");
    // 尝试调起网络监听器（异步）
    start_net_watcher();

    // 运行结束
    success("Engine run out.");
}
