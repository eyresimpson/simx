use std::fs;
use std::path::Path;

use crate::tools::log::shell::warn;

// 脚本引擎核心


// 执行脚本
pub fn exec_script(path: &str) {
    check_script_type(path);
    let path = Path::new(path);
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "py" => println!("Python script:"),
            "sh" => println!("Shell script: "),
            "bat" => println!("Batch script: "),
            "ps1" => println!("PowerShell script: "),
            "cmd" => println!("Command script: "),
            _ => println!("Other file type: "),
        }
    } else {
        // 不解析其他任何后缀名的文件
        return;
    }
}

// 加载并执行默认脚本
pub fn load_and_exec_default_script() {
    let path = "./scripts/";
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new(path).is_dir() {
        // 遍历文件夹下的所有内容
        traverse_folder(path);
    } else {
        warn("No default scripts found, Skip...")
    }
}

pub fn check_script_type(path: &str) {
    // 获取文件后缀名
    // println!("{}", path);
}

fn traverse_folder(folder_path: &str) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_path) = path.to_str() {
                        exec_script(file_path);
                    }
                } else if path.is_dir() {
                    if let Some(dir_path) = path.to_str() {
                        traverse_folder(dir_path);
                    }
                }
            }
        }
    }
}