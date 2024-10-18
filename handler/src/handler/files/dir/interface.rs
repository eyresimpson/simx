use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{info, warn};
use serde_json::Value;
use std::fs::{metadata, rename};
use std::path::Path;
use std::{fs, io};

pub fn handle_files_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建目录
        "create" => create_dir(node, flow_data),
        // 判断目录是否存在
        "exist" => exist_dir(node, flow_data),
        // 移动目录
        "mv" => mv_dir(node),
        // 复制目录
        "cp" => cp_dir(node, flow_data),
        // 删除目录
        "del" => del_dir(node),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 创建目录
pub fn create_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = path.as_str().expect("path must be string");
            let path = Path::new(&path);
            // 检查目录是否存在
            if metadata(path).is_ok() {
                info(format!("Path {} exist, skip...", path.display()).as_str());
                Ok(())
            } else {
                // 创建目录
                match fs::create_dir(path) {
                    Ok(_) => {
                        // 写到节点数据域
                        flow_data.nodes.insert(node.id.unwrap(), Value::from(path.display().to_string()));
                        Ok(())
                    }
                    // 目录创建失败
                    Err(_) => Err(NodeError::PathCreateError)
                }
            }
        }
        None => {
            // 找不到参数
            Err(NodeError::ParamNotFound("path".to_string()))
        }
    }
}

// 判断指定文件夹是否存在
pub fn exist_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = path.as_str().expect("path must be string");
            let path = Path::new(&path);
            // 检查目录是否存在
            if metadata(path).is_ok() {
                // 目录存在
                // info(format!("Path {} is exist.", path.display()).as_str());
                flow_data.nodes.insert(node.id.unwrap(), Value::from(true));
                Ok(())
            } else {
                // 目录不存在
                flow_data.nodes.insert(node.id.unwrap(), Value::from(false));
                Ok(())
            }
        }
        None => {
            // 找不到参数
            Err(NodeError::ParamNotFound("path".to_string()))
        }
    }
}

// 移动目录到新位置
pub fn mv_dir(node: Node) -> Result<(), NodeError> {
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


    match move_directory(source_path, target_path, true) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// 复制目录到新位置
pub fn cp_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
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

    copy_dir(source_path, target_path).expect("Cannot cp dir");
    println!("handler not support yet.{:?},{:?}", node, flow_data);
    Ok(())
}

// 删除目录
pub fn del_dir(node: Node) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = path.as_str().expect("path must be string");
            match remove_dir_all(path.as_ref()) {
                Ok(_) => Ok(()),
                Err(err) => Err(err)
            }
        }
        None => {
            Err(NodeError::ParamNotFound("path".to_string()))
        }
    }

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
            match remove_dir_all(destination_path) {
                Ok(_) => {}
                Err(e) => { return Err(e) }
            }
        } else {
            // 警告即可，无需退出
            warn(format!("target dir {} exist, skip...", destination).as_str())
        }
    }

    // 执行移动操作
    match rename(source_path, destination_path) {
        Ok(_) => Ok(()),
        Err(e) => { Err(NodeError::PathMoveError(e.to_string())) }
    }
}

// 用于递归删除目录
fn remove_dir_all(path: &Path) -> Result<(), NodeError> {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            remove_dir_all(&entry.path())?;
        }
    }
    match fs::remove_dir(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(NodeError::PathDeleteError)
    }
}

// 用于递归复制文件或文件夹
fn copy_dir(source_path: &Path, target_path: &Path) -> io::Result<()> {
    if source_path.is_dir() {
        // 如果源路径是一个目录，则递归复制目录及其内容
        fs::create_dir_all(target_path)?;
        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                // 递归复制子目录
                copy_dir(&entry.path(), &target_path.join(entry.file_name()))?;
            } else if file_type.is_file() {
                // 复制文件
                fs::copy(&entry.path(), &target_path.join(entry.file_name()))?;
            }
        }
    } else if source_path.is_file() {
        // 如果源路径是一个文件，直接复制文件
        fs::copy(source_path, target_path)?;
    }
    Ok(())
}