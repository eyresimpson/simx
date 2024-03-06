// 引擎核心
use crate::core::engine::env::{check, load_configuration};
use crate::core::engine::flow::load_and_exec_default_flow;
use crate::core::engine::watcher::{start_cron_watcher, start_net_watcher};
use crate::tools::log::shell::{info, success};

// 运行引擎
pub fn run() {
    info("Engine Starting...");
    // 检查工作环境（当前目录）
    check();
    // 尝试加载运行配置
    load_configuration();
    // 尝试调起网络监听器（如果需要）
    start_net_watcher();
    // 尝试调起CRON监听（如果需要）
    start_cron_watcher();
    // 尝试加载默认流
    load_and_exec_default_flow();
    success("Engine is currently running.");
    
}
