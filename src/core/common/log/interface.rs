use crate::core::common::log::direct::interface::{d_debug, d_err, d_info, d_script_err, d_script_log, d_success, d_warn};

pub fn info(text: &str) {
    d_info(text)
}

pub fn warn(text: &str) {
    d_warn(text)
}

pub fn fail(text: &str) {
    d_err(text)
}

pub fn success(text: &str) {
    d_success(text)
}

pub fn script_log(text: &str) {
    d_script_log(text)
}

pub fn script_fail(text: &str) {
    d_script_err(text)
}

pub fn debug(text: &str) {
    d_debug(text)
}
