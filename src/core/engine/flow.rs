use std::fs;
use std::path::Path;

use crate::core::flow::controller::interface::{exec_fl_flow, exec_json_flow, exec_toml_flow, exec_xml_flow};
use crate::tools::log::shell::info;

pub fn load_and_exec_default_flow() {
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new("init").join("flow").is_dir() {
        // 遍历文件夹下的所有内容
        traverse_folder(Path::new("init").join("flow").as_path());
    } else {
        info("No init flow found, Skip...")
    }
}

// 遍历并执行指定目录下的所有流程（包含子目录）
fn traverse_folder(folder_path: &Path) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        // 循环指定的目录
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    // 是一个文件，尝试作为脚本进行解析
                    exec_flow(path.as_path());
                } else if path.is_dir() {
                    // 多级文件夹，继续遍历其中的文件和文件夹
                    traverse_folder(path.as_path());
                }
            }
        }
    }
}

pub fn exec_flow(path: &Path) {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "flow"|"fl" => exec_fl_flow(path),
            "json" => exec_json_flow(path),
            "xml" => exec_xml_flow(path),
            "toml" => exec_toml_flow(path),
            // 目前拒绝处理其他类型的流程
            _ => return,
        }
    } else {
        // 不解析其他任何后缀名的文件
        return;
    }
}