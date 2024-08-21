// 标准文件日志

use crate::runtime::config::get_simx_config;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

// 写文件日志
pub fn write_log(log_message: &str, num: i64) {
    if get_log_num() < num {
        return;
    }
    let now = chrono::Local::now();
    // 日志目录，从配置文件中读取
    let conf = get_simx_config().engine;
    let path = conf.log_path;
    // 默认情况下是根据日期写日志，即每天都有一个日志
    let log_path = format!("{}/simx-{}.logger", path, now.format("%Y-%m-%d"));
    // 打开文件（如果不存在会自动创建）
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_path)
        .expect("Failed to open logger file");
    let mut writer = BufWriter::new(file);

    let formatted_time = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let str: &str;
    match num {
        0 => {
            str = "fail"
        }
        1 => {
            str = "warn"
        }
        2 => {
            str = "info"
        }
        3 => {
            str = "debug"
        }
        _ => {
            str = "info"
        }
    }
    let log_line = format!("{} [{}]: {} \n", formatted_time, str, log_message);
    match writer.write_all(log_line.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to write to logger file: {}", e);
        }
    }
}

fn get_log_num() -> i64 {
    let conf = get_simx_config().engine;
    let level = conf.file_log_level.as_str();
    // # 文件日志等级
    // 0 fail 仅写入错误日志
    // 1 warn：记录错误、警告日志
    // 2 info：记录错误、警告和信息日志
    // 3 debug: 包含每个节点的输入、输入等信息，最详细，仅适用于开发环境
    // 4 none: 不记录任何日志
    match level {
        "fail" => { 0 }
        "warn" => { 1 }
        "info" => { 2 }
        "debug" => { 3 }
        "none" => { -1 }
        _ => { 0 }
    }
}