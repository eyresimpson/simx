#[macro_use]
extern crate rocket;

use rocket::tokio;

use crate::core::engine::engine::run;
use crate::env::workspace::check;
use crate::tools::log::shell::{err, info, success};


mod tools;
mod core;
mod conf;
mod env;

#[tokio::main]
async fn main() {
    // 引擎运行前的准备和初始化动作
    let ret = init();
    if let Err(text) = ret {
        err(text.as_str());
        return;
    }
    success("Workspace check done.");

    // 尝试运行引擎（同步）
    run().await;
    // 程序运行结束后的清理动作
    clean();
}

fn init() -> Result<String, String> {
    // 检查工作环境（当前目录）
    return if let Err(_) = check() {
        Err("Check runtime env failed, Engine Shutdown.".parse().unwrap())
    } else {
        // success("");
        Ok("".to_string())
    };
}

fn clean() {
    info("Simx System Shutdown.");
}