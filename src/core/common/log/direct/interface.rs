use crate::conf::toml::get_engine_config;
use crate::core::common::log::direct::shell::basic::info as b_info;
use crate::core::common::log::direct::shell::basic::warn as b_warn;
use crate::core::common::log::direct::shell::basic::err as b_err;
use crate::core::common::log::direct::shell::basic::debug as b_debug;
use crate::core::common::log::direct::shell::basic::success as b_success;
use crate::core::common::log::direct::shell::colourful::script_log as b_script_log;
use crate::core::common::log::direct::shell::colourful::script_err as b_script_err;
use crate::core::common::log::direct::shell::colourful::info as c_info;
use crate::core::common::log::direct::shell::colourful::warn as c_warn;
use crate::core::common::log::direct::shell::colourful::debug as c_debug;
use crate::core::common::log::direct::shell::colourful::err as c_err;
use crate::core::common::log::direct::shell::colourful::success as c_success;
use crate::core::common::log::direct::shell::colourful::script_log as c_script_log;
use crate::core::common::log::direct::shell::colourful::script_err as c_script_err;


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
pub fn d_info(text: &str) {
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
pub fn d_success(text: &str) {
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
pub fn d_err(text: &str) {
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
pub fn d_warn(text: &str) {
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
pub fn d_script_log(text: &str) {
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
pub fn d_script_err(text: &str) {
    if get_log_num() < 0 {
        return;
    }
    if show_colourful() {
        c_script_err(text);
    } else {
        b_script_err(text);
    }
}

// 调试
pub fn d_debug(text: &str) {
    if get_log_num() < 3 {
        return;
    }
    if show_colourful() {
        c_debug(text);
    } else {
        b_debug(text);
    }
}