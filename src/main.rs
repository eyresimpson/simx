#[macro_use]
extern crate rocket;

use std::fs;
use std::path::Path;

use chrono::prelude::*;
use rocket::tokio;

use tools::log::interface::info;

use crate::core::engine::engine::run;
use crate::core::runtime::config::get_simx_config;
use crate::core::runtime::engine::set_engine_info;

mod core;
mod net;
mod tools;
mod test;
mod entity;

#[tokio::main]
async fn main() {
    // 引擎运行前的准备和初始化动作
    init();
    // 尝试运行引擎（同步）
    run().await;
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
