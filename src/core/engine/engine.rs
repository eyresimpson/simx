use crate::core::common::log::interface::{fail, info, success};
use crate::core::engine::init::engine_init;
use crate::core::engine::watcher::start_net_watcher;

/// 引擎核心
/// 其实引擎启动主要是启动了系统监听，引擎本身并不会持续运行，否则会占用一些不必要的资源，当有请求抵达监听器时，
/// 才会调用引擎方法，发起流程或脚本
pub async fn run() {
    info("Engine Starting...");

    // // 创建一个新线程
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("从新线程打印: {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // 
    // // 在主线程中执行其他操作
    // for i in 1..5 {
    //     println!("从主线程打印: {}", i);
    //     // thread::sleep(Duration::from_millis(1));
    // }
    // 
    // // 等待新线程结束
    // handle.join().unwrap();
    
    // debug("test");
    // 执行系统初始化事件
    // 包括运行初始化脚本和初始化流
    let init_ret = engine_init().await;
    if init_ret.is_err() {
        fail(init_ret.err().unwrap().as_str());
        return;
    }

    // 系统启动完成
    success("Engine has started.");

    // 尝试调起网络监听器（阻塞）
    start_net_watcher().await;

    // 运行结束
    // 如果是用户手动结束进程，不会执行到这里（只有系统主动结束此处才会执行）
    success("Engine run out.");
}
