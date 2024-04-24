use basic::err as b_err;
use basic::info as b_info;
use basic::script_err as b_script_err;
use basic::script_log as b_script_log;
use basic::success as b_success;
use basic::warn as b_warn;
use colourful::err as c_err;
use colourful::info as c_info;
use colourful::script_err as c_script_err;
use colourful::script_log as c_script_log;
use colourful::success as c_success;
use colourful::warn as c_warn;

use crate::conf::toml::get_engine_config;

mod basic;
mod colourful;


fn show_colourful() -> bool {
    let conf = get_engine_config();
    return conf.get("engine").unwrap().get("console-log-style").unwrap().as_bool().unwrap();
}

fn get_log_num() -> i64 {
    let conf = get_engine_config();
    let level = conf.get("engine").unwrap().get("shell-log-level").unwrap().as_str().unwrap();
    // # 控制台日志等级
    // 0 fail
    // 1 warn：仅显示
    // 2 info：显示引擎正在进行的步骤和操作
    // 3 debug: 包含每个节点的输入、输入等信息，最详细，仅适用于开发环境
    // 4 none
    match level {
        "fail" => { 0 }
        "warn" => { 1 }
        "info" => { 2 }
        "debug" => { 3 }
        "none" => { -1 }
        _ => { 0 }
    }
}

// 2级日志
pub fn info(text: &str) {
    if get_log_num() < 2 {
        return;
    }
    if show_colourful() {
        c_info(text);
    } else {
        b_info(text);
    }
}

// 2级日志
pub fn success(text: &str) {
    if get_log_num() < 2 {
        return;
    }
    if show_colourful() {
        c_success(text);
    } else {
        b_success(text);
    }
}

// 0级日志
pub fn err(text: &str) {
    if get_log_num() < 0 {
        return;
    }
    if show_colourful() {
        c_err(text);
    } else {
        b_err(text);
    }
}

// 1级日志
pub fn warn(text: &str) {
    if get_log_num() < 1 {
        return;
    }
    if show_colourful() {
        c_warn(text);
    } else {
        b_warn(text);
    }
}

// 脚本输出
// 2级日志
pub fn script_log(text: &str) {
    if get_log_num() < 2 {
        return;
    }
    if show_colourful() {
        c_script_log(text);
    } else {
        b_script_log(text);
    }
}

// 0级日志
pub fn script_err(text: &str) {
    if get_log_num() < 0 {
        return;
    }
    if show_colourful() {
        c_script_err(text);
    } else {
        b_script_err(text);
    }
}

// 流输出
// pub fn flow_log(text: &str) {
//     pcs!(Cyan => "➜ "; RBold, Cyan => "Script output: "; Cyan => text);
// }
//
// pub fn flow_err(text: &str) {
//     pcs!(Cyan => "➜ "; RBold, Cyan => "Script Error: "; Red => text);
// }

// // 流程引擎输出
// pub fn flow_log(text: &str){
//     pcs!(Green => "➜ "; RBold, RBlue => "Exec: "; RBlue => text);
// }