// 脚本引擎核心
use std::fs;
use std::path::Path;
use crate::core::exec::script::bat::exec_bat_script;

use crate::core::exec::script::py::exec_python_script;
use crate::core::exec::script::sh::exec_shell_script;
use crate::tools::log::shell::warn;

// 加载并执行默认脚本
pub fn load_and_exec_default_script() {
    let binding = Path::new("default").join("scripts");
    let path = binding.as_path();
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new(path).is_dir() {
        // 遍历文件夹下的所有内容
        traverse_folder(path);
    } else {
        warn("No default scripts found, Skip...")
    }
}

// 遍历指定文件夹并执行
fn traverse_folder(folder_path: &Path) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        // 循环指定的目录
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    // 是一个文件，尝试作为脚本进行解析
                    exec_script(path.as_path());
                } else if path.is_dir() {
                    // 多级文件夹，继续遍历其中的文件和文件夹
                    traverse_folder(path.as_path());
                }
            }
        }
    }
}

// 执行脚本
pub fn exec_script(path: &Path) {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "py" => exec_python_script(path),
            "sh" => exec_shell_script(path),
            "bat" => exec_bat_script(path),
            "ps1" => println!("PowerShell script"),
            "cmd" => println!("Command script"),
            "sql" => println!("SQL script"),
            _ => return,
        }
    } else {
        // 不解析其他任何后缀名的文件
        return;
    }
}