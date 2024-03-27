#[macro_use]
extern crate rocket;

use rocket::tokio;

use crate::core::engine::engine::run;
use crate::tools::log::shell::info;


mod tools;
mod core;
mod conf;

#[tokio::main]
async fn main() {
    // 尝试运行引擎（同步）
    run().await;
    info("Simx System Shutdown.");
}
