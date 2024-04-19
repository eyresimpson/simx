use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::core::common::log::shell::{err, warn};

pub fn handle_endpoint_file_plain(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    file_writer(data.clone(), args.clone())
}

fn file_writer(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    let file_path = args.get("path").unwrap();
    let mut file_content = String::new();
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
