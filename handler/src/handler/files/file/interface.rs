use crate::handler::files::common::operation::{common_copy, common_move, common_remove};
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
        "mv" => mv_file(node),
        // 复制文件
        "cp" => cp_file(node),
        // 删除文件
        "del" => del_file(node),

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

fn mv_file(node: Node) -> Result<(), NodeError> {
    let source_path = match node.attr.get("source") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("source".to_string())),
    };

    let target_path = match node.attr.get("target") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("target".to_string())),
    };

    let source_path = source_path.as_str().expect("source must be string");
    let target_path = target_path.as_str().expect("target must be string");


    match common_move(source_path, target_path, true) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn del_file(node: Node) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = path.as_str().expect("path must be string");
            match common_remove(path.as_ref()) {
                Ok(_) => {
                    println!("File deleted successfully.{}", path);
                    Ok(())
                }
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

fn cp_file(node: Node) -> Result<(), NodeError> {
    let source_path = match node.attr.get("source") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("source".to_string())),
    };

    let target_path = match node.attr.get("target") {
        Some(path) => path,
        None => return Err(NodeError::ParamNotFound("target".to_string())),
    };

    let source_path: &Path = source_path.as_str().expect("source must be string").as_ref();
    let target_path: &Path = target_path.as_str().expect("target must be string").as_ref();

    common_copy(source_path, target_path).expect("Cannot cp dir");
    Ok(())
}