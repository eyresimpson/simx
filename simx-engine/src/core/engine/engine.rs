use crate::core::engine::initialization::engine_init;
use crate::core::flow::interface::exec_flow;
use simx_common::log::interface::{fail, info, success};
use simx_common::runtime::extension::get_all_extension_info;
use simx_core::extension::interface::call_init;
use std::env;
use std::path::Path;

/// 引擎核心
/// 其实引擎启动主要是启动了系统监听，引擎本身并不会持续运行，否则会占用一些不必要的资源，当有请求抵达监听器时，
/// 才会调用引擎方法，发起流程或脚本
pub async fn serve() {
    info("Engine Starting...");

    // 执行系统初始化事件
    // 包括运行初始化脚本和初始化流
    let init_ret = engine_init().await;
    if init_ret.is_err() {
        fail(init_ret.err().unwrap().as_str());
        return;
    }

    // 系统启动完成
    success("Engine has started.");

    let mut jobs = vec![];

    // 获取插件列表
    let extensions = get_all_extension_info();
    // 遍历插件列表，调用init方法
    for extension in extensions {
        // 调用插件的init方法
        // 注意，新线程中执行init，init的执行结果的顺序不能保证
        let job = tokio::spawn(async move {
            call_init(extension).unwrap();
        });
        jobs.push(job);
    }

    // let (tx_a, rx_a) = mpsc::channel();
    // tx_a.send("1111111").unwrap();

    for job in jobs {
        // 只要有一个线程没有退出，就阻塞引擎不退出
        job.await.unwrap();
    }

    // 运行结束
    // 如果是用户手动结束进程，不会执行到这里（只有系统主动结束此处才会执行）
    info("Engine run out.");
}

/// 运行流
/// 此方法不会开启额外的线程，只是通过流引擎执行目标的流
pub async fn run() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 判断文件路径是否为空
    if args.len() <= 2 {
        fail("Please input flow file path.");
        return;
    }
    // 分析是否为flow文件（目前直接判断后缀名
    let path = Path::new(args[2].as_str());

    // 判断文件是否存在
    if !path.exists() {
        fail("The file is not exist.");
        return;
    }

    if !(path.extension().unwrap().to_str().unwrap() != ".flow") {
        fail("The flow file must be selected.");
        return;
    }


    // 调用流引擎执行该文件
    exec_flow(path).await;
}