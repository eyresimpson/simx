use std::fs::File;
use std::io::{Read, Write};

use crate::core::common::log::interface::{fail, warn};
use crate::core::flow::entity::standardisation::{Data, Node};

pub fn handle_file_plain(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        "reader" => {
            // 普通文本
            reader(node, flow_data);
        }
        "writer" => {
            // Json文件
            writer(node, flow_data);
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

// 此方法仅会触发一次，不是一个持续性的任务
fn reader(node: Node, flow_data: &mut Data) {
    let args = node.attr;
    let file_path = args.get("path").unwrap();
    // 返回的数据
    let read_file_ret = File::open(file_path);
    let mut file = read_file_ret.unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    // 将数据写入到flow_data中
    flow_data.data.insert("text".parse().unwrap(), toml_str);
}

fn writer(node: Node, flow_data: &mut Data) {
    let args = node.attr;
    let file_path = args.get("path").unwrap();
    let file_content;
    // 优先属性，其次是流内容中的 text 项
    if args.contains_key("text") {
        file_content = args.get("text").unwrap().to_string();
    } else {
        file_content = flow_data.data.get("text").unwrap().to_string();
    }

    let mut file = File::open(file_path);
    if file.is_err() {
        warn("Cannot find specified file, will create it.");
        file = File::create(file_path);
    }
    let ret = file.unwrap().write(file_content.as_ref());
    if ret.is_err() {
        fail("Cannot write content to file.");
        fail(ret.err().unwrap().to_string().as_str());
    }
}
