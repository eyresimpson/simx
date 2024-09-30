use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{info, success};
use std::fs;
use std::path::Path;

pub fn handle_files_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 创建目录
        "create" => create_dir(node, flow_data),
        // 判断目录是否存在
        "exist" => exist_dir(node, flow_data),
        // 移动目录
        "mv" => mv_dir(node, flow_data),
        // 复制目录
        "cp" => cp_dir(node, flow_data),
        // 目录授权
        "chmod" => chmod_dir(node, flow_data),
        // 删除目录
        "del" => del_dir(node, flow_data),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 创建目录
pub fn create_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let path_opt = node.attr.get("path");
    if path_opt.is_none() {
        return Err(NodeError::ParamNotFound("path".to_string()));
    }
    let path = path_opt.unwrap();
    let path = Path::new(path);

    // 检查目录是否存在
    if fs::metadata(path).is_ok() {
        info(format!("Path {} exist, skip...", path.display()).as_str());
        Ok(())
    } else {
        // 创建目录
        fs::create_dir(path).expect("Make dir failed");
        success(format!("path {} make success", path.display()).as_str());
        Ok(())
    }
}

// 判断指定文件夹是否存在
pub fn exist_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
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
pub fn mv_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
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
pub fn del_dir(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    Ok(())
}