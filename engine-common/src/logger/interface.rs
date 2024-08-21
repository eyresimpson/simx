use crate::logger::direct::interface::{d_debug, d_err, d_info, d_script_err, d_script_log, d_success, d_warn};
use crate::logger::file::slog::write_log;

pub fn info(text: &str) {
    d_info(text);
    write_log(text, 2);
}

pub fn warn(text: &str) {
    d_warn(text);
    write_log(text, 1);
}

pub fn fail(text: &str) {
    d_err(text);
    write_log(text, 0);
}

pub fn success(text: &str) {
    d_success(text);
    write_log(text, 2);
}

pub fn script_log(text: &str) {
    d_script_log(text);
    write_log(text, 2);
}

pub fn script_fail(text: &str) {
    d_script_err(text);
    write_log(text, 0);
}

pub fn debug(text: &str) {
    d_debug(text);
    write_log(text, 3);
}
