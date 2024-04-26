// 脚本引擎核心
use std::fs;
use std::path::Path;
use crate::conf::runtime::get_runtime_conf;

use crate::core::common::log::interface::info;
use crate::core::script::bat::exec_bat_script;
use crate::core::script::ps1::exec_powershell_script;
use crate::core::script::py::exec_python_script;
use crate::core::script::sh::exec_shell_script;
use crate::core::script::sql::exec_sql_script;

// 加载并执行默认脚本
pub fn load_and_exec_default_script() {
    let script_path = get_runtime_conf("script_path").unwrap();
    // TODO: 将这个路径修改到配置文件中
    let binding = Path::new(script_path.as_str()).join("init");
    let path = binding.as_path();
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new(path).is_dir() {
        // 遍历文件夹下的所有内容
        traverse_folder(path);
    } else {
        info("No init scripts found, Skip...")
    }
}

// 遍历并执行指定目录下的所有脚本（包含子目录）
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

// 执行指定路径下的脚本
pub fn exec_script(path: &Path) {
    if let Some(extension) = path.extension() {
        match extension.to_str().unwrap().to_lowercase().as_str() {
            "py" => exec_python_script(path),
            "sh" => exec_shell_script(path),
            "bat" | "cmd" => exec_bat_script(path),
            "ps1" => exec_powershell_script(path),
            "sql" => exec_sql_script(path),
            // 目前拒绝处理其他脚本
            _ => return,
        }
    } else {
        // 不解析其他任何后缀名的文件
        return;
    }
}