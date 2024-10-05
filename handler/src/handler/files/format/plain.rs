use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn handle_file_plain(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 读取器
        "reader" => reader(node, flow_data),
        // 写入器
        "writer" => writer(node, flow_data),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 此方法仅会触发一次，不是一个持续性的任务
fn reader(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let args = node.attr;
    let file_path = args.get("path").unwrap();
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(NodeError::FileNotFound);
    }
    // 返回的数据
    let read_file_ret = File::open(file_path);
    let mut file = read_file_ret.unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    // 将数据写入到flow_data中
    flow_data.data.insert("text".parse().unwrap(), toml_str.into_bytes());
    Ok(())
}

fn writer(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let args = node.attr;
    let file_path = args.get("path").unwrap();
    let file_content: &[u8];
    let bytes;
    // 优先属性，其次是流内容中的 text 项
    if args.contains_key("text") {
        bytes = args.get("text").unwrap().to_string().into_bytes();
        file_content = &*bytes;
    } else {
        let file_content_opt = flow_data.data.get("text");
        if file_content_opt.is_none() {
            return Err(NodeError::ParamNotFound("content data text".to_string()))
        }
        // 判断错误逻辑，可能这部分也没有值
        file_content = flow_data.data.get("text").unwrap();
    }

    let mut file = File::open(file_path);
    if file.is_err() {
        // 没有这个文件，直接创建个新的文件
        file = File::create(file_path);
    } else {
        if args.contains_key("override") && args.get("override").expect("Cannot get node 'override' attr.").as_str().eq("force") {
            // 删除指定文件并重建
            file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file_path);
        } else {
            // 其他任何情况都进行末尾附加操作
            file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_path);
        }
    }
    let ret = file.unwrap().write(file_content.as_ref());
    if ret.is_err() {
        return Err(NodeError::FileWriteError(ret.err().unwrap().to_string()))
    }
    Ok(())
}
