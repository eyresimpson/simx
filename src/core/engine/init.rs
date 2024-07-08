use crate::conf::runtime::set_runtime_conf;
use crate::conf::toml::get_engine_config;
use crate::core::common::log::interface::{info, success};
use crate::core::env::check::env_check;
use crate::core::extension::interface::load_local_extendion;
use crate::core::flow::interface::load_and_exec_default_flow;
use crate::core::script::interface::load_and_exec_default_script;
use crate::db::controller::db_init;

pub async fn engine_init() -> Result<String, String> {
    // 系统引擎配置
    let engine_conf = get_engine_config();
    // 从配置文件中初始化系统文件夹位置
    set_runtime_conf("flow_path", engine_conf.get("engine").unwrap().get("flow-path").unwrap().as_str().unwrap());
    set_runtime_conf("script_path", engine_conf.get("engine").unwrap().get("script-path").unwrap().as_str().unwrap());
    set_runtime_conf("db_path", engine_conf.get("engine").unwrap().get("db-path").unwrap().as_str().unwrap());
    set_runtime_conf("log_path", engine_conf.get("engine").unwrap().get("log-path").unwrap().as_str().unwrap());
    set_runtime_conf("ext_path", engine_conf.get("engine").unwrap().get("ext-path").unwrap().as_str().unwrap());

    // 检查工作环境（当前目录）
    let env_check_ret = env_check();
    // 判断环境检查是否通过
    if env_check_ret.is_err() {
        return Err("Check Engine Runtime Env Failed.".to_string());
    }

    // 扫描并加载插件
    load_local_extendion();

    // TODO：检查文件日志大小，如果超过指定大小，就进行压缩并放入backup文件夹中，默认按照日期进行分类（年月日时）

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
        load_and_exec_default_flow().await;
        success("Run init flow done.");
    } else {
        info("Skip init flow running.");
    }

    // 返回成功信息
    return Ok("Engine init success.".parse().unwrap());
}