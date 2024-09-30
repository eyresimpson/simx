use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{info, success, warn};
use std::fs;
use std::fs::{metadata, rename};
use std::path::Path;

pub fn handle_files_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建目录
        "create" => create_dir(node),
        // 判断目录是否存在
        "exist" => exist_dir(node),
        // 移动目录
        "mv" => mv_dir(node),
        // 复制目录
        "cp" => cp_dir(node, flow_data),
        // 目录授权
        "chmod" => chmod_dir(node, flow_data),
        // 删除目录
        "del" => del_dir(node),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 创建目录
pub fn create_dir(node: Node) -> Result<(), NodeError> {
    let path_opt = node.attr.get("path");
    if path_opt.is_none() {
        return Err(NodeError::ParamNotFound("path".to_string()));
    }
    let path = path_opt.unwrap();
    let path = Path::new(path);

    // 检查目录是否存在
    if metadata(path).is_ok() {
        info(format!("Path {} exist, skip...", path.display()).as_str());
        Ok(())
    } else {
        // 创建目录
        fs::create_dir(path).expect("Make dir failed");
        success(format!("path {} make success", path.display()).as_str());
        // flow_data.nodes.insert(node., "true".as_bytes().to_vec());
        Ok(())
    }
}

// 判断指定文件夹是否存在
pub fn exist_dir(node: Node) -> Result<(), NodeError> {
    let path_opt = node.attr.get("path");
    if path_opt.is_none() {
        return Err(NodeError::ParamNotFound("path".to_string()));
    }
    let path = path_opt.unwrap();
    let path = Path::new(path);
    // 检查目录是否存在
    if fs::metadata(path).is_ok() {
        info(format!("Path {} is exist.", path.display()).as_str());
        Ok(())
    } else { Ok(()) }
}

// 移动目录到新位置
pub fn mv_dir(node: Node) -> Result<(), NodeError> {
    let source_path_opt = node.attr.get("source");
    let target_path_opt = node.attr.get("target");
    let overwrite_opt = node.attr.get("overwrite");
    node.attr.get("force");

    if source_path_opt.is_none() || target_path_opt.is_none() {
        return Err(NodeError::ParamNotFound("source or target".to_string()));
    }
    let ret = move_directory(
        source_path_opt.unwrap(),
        target_path_opt.unwrap(),
        if (overwrite_opt.is_none()) { false } else if overwrite_opt.unwrap() == "true" { true } else { false },
    );
    if ret.is_err() {
        return Err(NodeError::PathMoveError);
    }
    Ok(())
}

// 复制目录到新位置
pub fn cp_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

// 为目录授权（仅Linux）
pub fn chmod_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}

// 删除目录
pub fn del_dir(node: Node) -> Result<(), NodeError> {
    let path_opt = node.attr.get("path");
    if path_opt.is_none() {
        return Err(NodeError::ParamNotFound("path".to_string()));
    }
    let ret = remove_dir_all(path_opt.unwrap().as_ref());
    if ret.is_err() {
        return Err(NodeError::PathDeleteError);
    }
    Ok(())
}

fn move_directory(source: &str, destination: &str, overwrite: bool) -> Result<(), NodeError> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    // 检查源目录是否存在
    if !metadata(source_path).is_ok() {
        return Err(NodeError::PathNotFound);
    }

    // 检查目标位置是否已存在
    if metadata(destination_path).is_ok() {
        if overwrite {
            // 强制模式下删除目标位置的内容
            let ret = remove_dir_all(destination_path);
            if ret.is_err() {
                return ret
            }
        } else {
            // 警告即可，无需退出
            warn(format!("target dir {} exist, skip...", destination).as_str())
        }
    }

    // 执行移动操作
    let ret = rename(source_path, destination_path);
    if ret.is_err() {
        return Err(NodeError::PathMoveError);
    }
    Ok(())
}

// 用于递归删除目录
fn remove_dir_all(path: &Path) -> Result<(), NodeError> {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            remove_dir_all(&entry.path())?;
        }
    }
    let ret = fs::remove_dir(path);
    if ret.is_err() {
        Err(NodeError::PathDeleteError)
    } else {
        Ok(())
    }
}