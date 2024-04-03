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

fn init() {info("Simx System Starting...");}

fn clean() {
    info("Simx System Shutdown.");
}
