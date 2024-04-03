use crate::conf::simx::get_engine_config;
use crate::core::db::controller::db_init;
use crate::core::engine::flow::load_and_exec_default_flow;
use crate::core::engine::script::load_and_exec_default_script;
use crate::core::env::check::env_check;
use crate::tools::log::shell::{info, success};

pub fn engine_init() -> Result<String, String> {
    let engine_conf = get_engine_config();
    // 检查工作环境（当前目录）
    let env_check_ret = env_check();
    // 判断环境检查是否通过
    if env_check_ret.is_err() {
        return Err("Check Engine Runtime Env Failed.".to_string());
    }

    // 尝试检查并初始化数据库
    info("System Database checking...");
    if db_init().is_err() {
        return Err("System Error: Check Your Db Conf!".to_string());
    } else {
        success("System database checked successfully.");
    }

    // 初始化脚本
    if engine_conf.get("engine").unwrap().get("run-init-script").unwrap().as_bool().unwrap() {
        info("Default script running...");
        load_and_exec_default_script();
        success("Run init script done.");
    } else {
        info("Skip init script running.");
    }

    // 初始流
    if engine_conf.get("engine").unwrap().get("run-init-flow").unwrap().as_bool().unwrap() {
        info("Default flow running...");
        load_and_exec_default_flow();
        success("Run init flow done.");
    } else {
        info("Skip init flow running.");
    }

    // 返回成功信息
    return Ok("Engine init success.".parse().unwrap());
}