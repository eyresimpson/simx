use crate::core::engine::initialization::engine_init;
use crate::core::extension::interface::call_init;
use crate::core::flow::interface::exec_flow;
use crate::core::runtime::extension::get_all_extension_info;
use crate::tools::log::interface::{fail, info, success};
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
        // 注意，新线程中执行init
        let job = tokio::spawn(async move {
            call_init(extension).unwrap();
        });
        jobs.push(job);
    }

    for job in jobs {
        job.await.unwrap();
    }

    // 尝试调起网络监听器（阻塞）
    // start_net_watcher().await;

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
    if !(path.extension().unwrap().to_str().unwrap() != ".flow") {
        fail("The flow file must be selected.");
        return;
    }

    // 判断文件是否存在
    if !path.exists() {
        fail("The file is not exist.");
        return;
    }
    // 重新加载插件信息，避免流程找不到插件
    // let engine_conf = get_simx_config().engine;
    //
    // let ext_path = engine_conf.ext_path;
    // reload_local_traverse_folder(Path::new(ext_path.as_str()), "ext");

    // 调用流引擎执行该文件
    exec_flow(path).await;
}