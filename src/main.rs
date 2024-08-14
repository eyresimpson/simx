#[macro_use]
extern crate rocket;

use chrono::prelude::*;
use rocket::tokio;

use tools::log::interface::info;

use crate::conf::runtime::set_runtime_conf;
use crate::core::engine::engine::run;

mod core;
mod conf;
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
    set_runtime_conf("engine_version", engine_version);
    set_runtime_conf("support_api_version", support_api_version);
    set_runtime_conf("engine_start_datetime", local.to_string().as_str());
    info("Simx System Starting...");
}

// 这个是为了后续的内存池清理工作准备的地方，有时间补充一下吧...
fn clean() {
    info("Simx System Shutdown.");
}
