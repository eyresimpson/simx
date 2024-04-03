use crate::core::engine::init::engine_init;
use crate::core::engine::watcher::start_net_watcher;
use crate::tools::log::shell::{err, info, success};

/// 引擎核心
pub async fn run() {
    info("Engine Starting...");

    // 执行系统初始化事件
    // 包括运行初始化脚本和初始化流
    let init_ret = engine_init();
    if init_ret.is_err() {
        err(init_ret.err().unwrap().as_str());
        return;
    }

    // 系统启动完成
    success("Engine has started.");

    // 尝试调起网络监听器（阻塞）
    start_net_watcher().await;

    // 运行结束
    success("Engine run out.");
}
