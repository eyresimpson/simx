use crate::core::engine::initialization::engine_init;
use engine_common::logger::interface::{fail, info, success};
use engine_common::runtime::extension::get_all_extension_info;
use engine_common::thread::flow::exec_flow;
use engine_handler::extension::interface::call_init;
use std::env;
use std::path::Path;
use engine_common::runtime::config::get_simx_config;
use crate::core::flow::dispatch::interface::dispatch_flow;

/// 引擎核心
/// 其实引擎启动主要是启动了系统监听，引擎本身并不会持续运行，否则会占用一些不必要的资源，当有请求抵达监听器时，
/// 才会调用引擎方法，发起流程或脚本
pub async fn serve() {
    let simx_config = get_simx_config();
    println!(
        " _______ _______ _______ ___ ___ 
|     __|_     _|   |   |   |   |
|__     |_|   |_|       |-     -|
|_______|_______|__|_|__|___|___|",
    );
    info("Engine Starting...");

    // 执行系统初始化事件
    // 包括运行初始化脚本和初始化流
    let init_ret = engine_init().await;
    if init_ret.is_err() {
        fail(init_ret.err().unwrap().as_str());
        return;
    }

    dispatch_flow("D:\\Code\\simx\\example\\flow\\init\\hello.flow".as_ref(), "".to_string());


    // 系统启动完成
    success("Engine has started.");

    let mut jobs = vec![];

    // 获取插件列表
    let extensions = get_all_extension_info();
    // 遍历插件列表，调用init方法
    for extension in extensions {
        if extension.init_func.is_empty() {
            // 如果找不到初始化方法，则跳过插件的初始化（并不强制所有插件必须有初始化方法）
            continue;
        }
        // 调用插件的init方法
        // 注意，新线程中执行init，init的执行结果的顺序不能保证
        let job = tokio::spawn(async move {
            call_init(extension).unwrap();
        });
        jobs.push(job);
    }

    for job in jobs {
        // 只要有一个线程没有退出，就阻塞引擎不退出
        job.await.unwrap();
    }

    info("Engine is running, If you want to exit, press Ctrl + C");

    // 检查配置中是否需要阻塞进程
    if simx_config.engine.run_strategy != "once" {
        // 等待用户 ctrl + c 结束进程
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
        }
    }

    // 运行结束
    // 如果是用户手动结束进程，不会执行到这里（只有系统主动结束此处才会执行）
    info("Engine run out.");
}

/// 运行流
/// 此方法不会开启额外的线程，只是通过流引擎执行目标的流
pub fn run() {
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
    exec_flow(path.to_str().unwrap().to_string());
}
