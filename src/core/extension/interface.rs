use std::fs;
use std::io::Read;
use std::path::Path;
use rusqlite::Connection;
use serde_json::from_str;

use crate::conf::runtime::{get_runtime_conf, set_runtime_conf};
use crate::core::common::log::interface::info;
use crate::entity::ext::Extension;

// 加载配置目录下的插件信息
pub fn load_local_extensions() -> Result<String, String> {
    info("Load Extension...");
    let script_path = get_runtime_conf("ext_path").unwrap();
    let binding = Path::new(script_path.as_str()).iter().as_path();
    let path = binding;
    // 默认脚本指在运行目录同级下的script/ 中的所有脚本文件（py/sh/bat/cmd/ps1），根据操作系统类型执行对应的脚本文件
    // 检查运行目录是否有对应的文件夹
    if Path::new(path).is_dir() {
        // 遍历文件夹下的所有内容
        traverse_folder(path);
        Ok("Load Extensions Done.".parse().unwrap())
    } else {
        info("No extensions found, Skip...");
        Ok("No extensions found, Skip...".parse().unwrap())
    }
}

// 遍历并执行指定目录下的所有路径（包含子目录）
fn traverse_folder(folder_path: &Path) {
    if let Ok(entries) = fs::read_dir(folder_path) {
        // 循环指定的目录
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    // 是一个文件，尝试作为脚本进行解析
                    load_extension_by_path(path.as_path())
                } else if path.is_dir() {
                    // 多级文件夹，继续遍历其中的文件和文件夹
                    traverse_folder(path.as_path());
                }
            }
        }
    }
}

// 此方法根据一个url加载插件（相当于从远程下载）
// pub fn load_extension_by_url() {}

// 此方法根据路径加载插件，注意是通过文件后缀名判断插件类型，因此必须有正确地插件名称
pub fn load_extension_by_path(path: &Path) {
    // 判断插件类型
    if path.exists() {
        if path.file_name().unwrap().to_str().unwrap().eq("extension.json") {
            // 数据库地址
            let db_path = get_runtime_conf("db_path").unwrap();
            // 连接到数据库（获取到的信息需要写入到数据库中）
            let conn = Connection::open(format!("./{}/simx.db", db_path)).unwrap();
            // 循环指定的目录
            let table_name = "simx_ext".to_string();
            // 读取 JSON 文件
            let file_path = Path::new(path);
            let data = fs::read_to_string(file_path).expect("Unable to read file");
            let extension: Extension = from_str(&data).expect("JSON was not well-formatted");
            
            // 将数据放到 runtime 配置中
            set_runtime_conf(extension.name.as_str(), serde_json::to_string(&extension).unwrap().as_str());
            let sql = format!("insert into {} (name, version, path, description, license, author, keywords, dependencies, function) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);", table_name);
            let _ = conn.execute(sql.as_str(), &[
                extension.name.as_str(),
                extension.version.as_str(),
                path.to_str().unwrap(),
                extension.description.as_str(),
                extension.license.as_str(),
                extension.author.as_str(),
                extension.keywords.join(",").as_str(),
                serde_json::to_string(&extension.dependencies).unwrap().as_str(),
                serde_json::to_string(&extension.function).unwrap().as_str(),
            ]);
            conn.close().unwrap()
        }
    }
}

//
// // 检查插件可用性
// pub fn check_extension_available() {}
//
// // 获取指定插件信息
// pub fn get_extension_info() {}
//
// // 获取所有插件信息
// pub fn get_all_extension_info() {}
//
// // 获取指定插件的详细信息
// pub fn get_extension_detail() {}
//
// // 卸载指定插件
// pub fn unload_extension() {}
//
// // 删除指定插件
// pub fn delete_extension() {}
//
// // 获取指定插件功能列表
// pub fn get_extension_func(){
//
// }
//
// // 执行目标插件的目标功能
// pub fn execute_extension_func(){
//
// }