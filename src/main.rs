#[macro_use]
extern crate rocket;

use rocket::tokio;

use crate::core::engine::engine::run;
use crate::tools::log::shell::info;

mod tools;
mod core;
mod conf;
mod net;
mod db;

#[tokio::main]
async fn main() {
    // 引擎运行前的准备和初始化动作
    init();
    // 尝试运行引擎（同步）
    run().await;
    // 程序运行结束后的清理动作
    clean();
}

// 目前情况下暂时不需要在初始化时做什么操作，后续可能会考虑弄点什么东西放着
fn init() {info("Simx System Starting...");}

// 这个是为了后续的内存池清理工作准备的地方，有时间补充一下吧...
fn clean() {
    info("Simx System Shutdown.");
}
