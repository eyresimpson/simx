use crate::core::engine::engine::{run, serve};
use crate::core::engine::thread::init_thread_monitor;
use engine_common::logger::interface::info;
use engine_common::runtime::config::get_simx_config;
use engine_common::runtime::engine::set_engine_info;
use std::env;
use std::fs;
use std::path::Path;

mod core;
mod tools;

#[tokio::main]
async fn main() {
    // it_works();
    // 引擎运行前的准备和初始化动作
    init();
    // 分析用户输入参数，如果没有输入参数，就代表默认的启动方式
    // 获取命令行参数
    
    let args: Vec<String> = env::args().collect();
    // 如果没有输入参数
    if args.len() > 1 {
        // 解析输入参数
        match args[1].as_str() {
            "serve" => serve().await,
            "run" => run(),
            _ => run(),
        }
        return;
    } else {
        // 同步运行监听
        serve().await;
    }
    // 程序运行结束后的清理动作
    // 注意，用户手动结束进程不会触发此方法
    clean();
}

// 初始化方法
fn init() {
    // 每次更新系统都记得修改这里
    let engine_version = "1.0.0";
    // 系统支持API的版本
    let support_api_version = "0.0.1";
    set_engine_info("engine_version", engine_version);
    set_engine_info("support_api_version", support_api_version);
    // 检查日志文件夹
    let engine_conf = get_simx_config().engine;
    //  初始化核心线程池
    init_thread_monitor();
    // 检查运行目录下是否有日志目录
    let log_path = Path::new(engine_conf.log_path.as_str()).is_dir();
    if !log_path {
        // 重建日志目录
        fs::create_dir(engine_conf.log_path.as_str()).expect("Engine cannot fix workspace, Please check your environment.");
        // info("Cannot find logs dir, system will automatically rebuild this directory.");
    }
    info("Simx engine ready.");
}

// 这个是为了后续的内存池清理工作准备的地方，有时间补充一下吧...
fn clean() {
    info("Simx engine jobs complete.");
}
