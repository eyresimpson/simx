#[macro_use]
extern crate rocket;

use rocket::tokio;

use crate::core::engine::engine::run;


mod tools;
mod core;
mod conf;

#[tokio::main]
async fn main() {
    // 尝试运行引擎（同步）
    run().await;

}
