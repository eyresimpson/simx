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

pub fn info(text: &str) {
    if show_colourful() {
        c_info(text);
    } else {
        b_info(text);
    }
}

pub fn success(text: &str) {
    if show_colourful() {
        c_success(text);
    } else {
        b_success(text);
    }
}

pub fn err(text: &str) {
    if show_colourful() {
        c_err(text);
    } else {
        b_err(text);
    }
}

pub fn warn(text: &str) {
    if show_colourful() {
        c_warn(text);
    } else {
        b_warn(text);
    }
}

// 脚本输出
pub fn script_log(text: &str) {
    if show_colourful() {
        c_script_log(text);
    } else {
        b_script_log(text);
    }
}

pub fn script_err(text: &str) {
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