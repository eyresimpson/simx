use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use crate::core::common::log::shell::{err, warn};

pub fn handle_origin_file_plain(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    file_reader(data.clone(), args.clone())
}

// 此方法仅会触发一次，不是一个持续性的任务
fn file_reader(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    let file_path = args.get("path").unwrap();
    // 返回的数据
    let mut ret: HashMap<String, String> = data;
    let read_file_ret = File::open(file_path);
    let mut file = read_file_ret.unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    ret.insert("text".parse().unwrap(), toml_str);
    return ret;
}

fn file_writer(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    let file_path = args.get("path").unwrap();
    let file_content;
    // 优先属性，其次是流内容中的 text 项
    if args.contains_key("content") {
        file_content = args.get("content").unwrap().to_string();
    } else {
        file_content = data.get("text").unwrap().to_string();
    }

    let mut file = File::open(file_path);
    if file.is_err() {
        warn("Cannot find specified file, will create it.");
        file = File::create(file_path);
    }
    let ret = file.unwrap().write(file_content.as_ref());
    if ret.is_err() {
        err("Cannot write content to file.");
        err(ret.err().unwrap().to_string().as_str());
    }
    return data;
}
