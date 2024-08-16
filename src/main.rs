#[macro_use]
extern crate rocket;

use std::env;
use std::fs;
use std::path::Path;

use chrono::prelude::*;
use rocket::tokio;

use tools::log::interface::info;

use crate::core::engine::engine::{run, serve};
use crate::core::runtime::config::get_simx_config;
use crate::core::runtime::engine::set_engine_info;

mod core;
mod tools;
mod test;
mod entity;


#[tokio::main]
async fn main() {
    // 引擎运行前的准备和初始化动作
    init();
    // 分析用户输入参数，如果没有输入参数，就代表默认的启动方式
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    // 如果没有输入参数
    if args.len() > 1 {
        // serve            启动监听服务，无参数也会作为server启动
        // run [file path]  通过流引擎运行目标flow文件
        // 解析输入参数
        match args[1].as_str() {
            "serve" => serve().await,
            "run" => run().await,
            _ => println!("无效的参数：{}", args[1]),
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
    let local: DateTime<Local> = Local::now();
    set_engine_info("engine_version", engine_version);
    set_engine_info("support_api_version", support_api_version);
    set_engine_info("engine_start_datetime", local.to_string().as_str());
    // 检查日志文件夹
    let engine_conf = get_simx_config().engine;
    // 检查运行目录下是否有日志目录
    let log_path = Path::new(engine_conf.log_path.as_str()).is_dir();
    if !log_path {
        // 重建日志目录
        fs::create_dir(engine_conf.log_path.as_str()).expect("Engine cannot fix workspace, Please check your environment.");
        // info("Cannot find logs dir, system will automatically rebuild this directory.");
    }
    info("Simx System Starting...");
}

// 这个是为了后续的内存池清理工作准备的地方，有时间补充一下吧...
fn clean() {
    info("Simx System Shutdown.");
}
