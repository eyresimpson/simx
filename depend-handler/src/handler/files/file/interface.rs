use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub fn handle_files_file(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建文件
        "create" => {
            touch_file(node.attr.get("path").unwrap().as_str()).expect("Cannot create file.");
        }
        // 写文件（字符串）
        "write_str" => {}
        // 写文件（二进制）
        "write" => {}
        // 读文件（字符串）
        "read_str" => {}
        // 读文件（二进制）
        "read" => {}
        // 判断文件是否存在
        "exist" => {}
        // 移动目录
        "mv" => {}
        // 复制目录
        "cp" => {}
        // 目录授权
        "chmod" => {}
        // 删除目录
        "del" => {}
        _ => {
            // 找不到，一般是用户写错了，或者设计器和引擎版本不兼容
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

fn touch_file(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);

    // 尝试打开文件，如果不存在则创建新文件
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map(|_| ())
        .or_else(|err| {
            if err.kind() == io::ErrorKind::AlreadyExists {
                // 如果文件已存在，则直接打开文件
                File::open(&path).map(|_| ())
            } else {
                Err(err)
            }
        })?;

    Ok(())
}