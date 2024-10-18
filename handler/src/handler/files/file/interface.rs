use crate::handler::files::common::operation::remove_dir_all;
use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

pub fn handle_files_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建文件
        "create" => {
            make_file(node, flow_data)
        }
        // 写文件（字符串）
        "write_str" => { Ok(()) }
        // 写文件（二进制）
        "write" => { Ok(()) }
        // 读文件（字符串）
        "read_str" => { Ok(()) }
        // 读文件（二进制）
        "read" => { Ok(()) }
        // 判断文件是否存在
        "exist" => { Ok(()) }
        // 移动文件
        "mv" => { Ok(()) }
        // 复制文件
        "cp" => { Ok(()) }
        // 删除文件
        "del" => {
            match node.attr.get("path") {
                Some(path) => {
                    let path = path.as_str().expect("path must be string");
                    match remove_dir_all(path.as_ref()) {
                        Ok(_) => {
                            println!("File deleted successfully.{}", path);
                            Ok(())
                        },
                        Err(err) => {
                            println!("File deleted F.{:?}", err);
                            Err(err)
                        }
                    }
                }
                None => {
                    Err(NodeError::ParamNotFound("path".to_string()))
                }
            }
        }
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

fn make_file(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let path = match node.attr.get("path") {
        Some(path) => path.as_str().unwrap(),
        None => return Err(NodeError::ParamNotFound("path".to_string()))
    };

    let result = touch_file(path);
    if result.is_err() {
        result
    } else {
        // 如果执行成功，就将路径写入到节点数据域
        flow_data.nodes.insert(node.id.unwrap(), path.parse().unwrap());
        Ok(())
    }
}

fn touch_file(file_path: &str) -> Result<(), NodeError> {
    let path = Path::new(file_path);

    // 尝试打开文件，如果不存在则创建新文件
    let ret = OpenOptions::new()
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
        });
    if ret.is_err() {
        return Err(NodeError::FileCreateError);
    }
    Ok(())
}