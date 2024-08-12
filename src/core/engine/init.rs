use std::fs;
use std::path::Path;
use rusqlite::Connection;
use serde_json::from_str;
use crate::conf::runtime::{get_runtime_conf, set_runtime_conf};
use crate::conf::toml::get_engine_config;
use crate::core::common::log::interface::{fail, info, success, warn};
use crate::core::env::check::env_check;
use crate::core::flow::interface::load_and_exec_default_flow;
use crate::core::script::interface::load_and_exec_default_script;
use crate::db::controller::db_init;
use crate::entity::ext::Extension;

pub async fn engine_init() -> Result<String, String> {
    // 系统引擎配置
    let engine_conf = get_engine_config();
    // 运行模式
    set_runtime_conf("engine-mode", engine_conf.get("engine").unwrap().get("engine-mode").unwrap().as_str().unwrap());
    // 流所在目录
    set_runtime_conf("flow_path", engine_conf.get("engine").unwrap().get("flow-path").unwrap().as_str().unwrap());
    // 脚本目录
    set_runtime_conf("script_path", engine_conf.get("engine").unwrap().get("script-path").unwrap().as_str().unwrap());
    // 文件数据库目录
    set_runtime_conf("db_path", engine_conf.get("engine").unwrap().get("db-path").unwrap().as_str().unwrap());
    // 日志目录
    set_runtime_conf("log_path", engine_conf.get("engine").unwrap().get("log-path").unwrap().as_str().unwrap());
    // 插件所在目录
    set_runtime_conf("ext_path", engine_conf.get("engine").unwrap().get("ext-path").unwrap().as_str().unwrap());
    // 初始化内存中的脚本集合
    set_runtime_conf("script_list", "[]");
    // 初始化内存在的流集合
    set_runtime_conf("flow_list", "[]");


    // 检查运行模式
    match get_runtime_conf("engine-mode").unwrap().as_str() {
        "memory" => {
            info("Engine run in [ Memory ] mode");
        }
        "file-db" => {
            info("Engine run in [ File Database ] mode");
        }
        "mixture" => {
            info("Engine run in [ Mixture ] mode");
        }
        "db" => {
            info("Engine run in [ Database ] mode");
        }
        _ => {
            info("Engine run in [ Memory ] mode");
        }
    }

    // 判断运行模式，如果非内存模式的情况下再初始化数据库
    if !get_runtime_conf("engine-mode").unwrap().eq("memory") {
        // 尝试检查并初始化数据库
        info("System Database checking...");
        if db_init().is_err() {
            return Err("System Error: Check Your Db Conf!".to_string());
        } else {
            success("System database checked successfully.");
        }
    }

    // 扫描并加载插件
    // 插件的扫描和加载要早于环境检查和流程运行，但要晚于数据库初始化
    // if load_local_extensions().is_err() {
    //     return Err("Cannot load local extensions.".to_string());
    // }else{
    //     success("Load local extensions done.");
    // }

    // 检查工作环境（当前目录）
    let env_check_ret = env_check();
    // 判断环境检查是否通过
    if env_check_ret.is_err() {
        return Err("Check Engine Runtime Env Failed.".to_string());
    }

    // 尝试扫描并加载流，默认全部
    let result = scan_load_local("");
    if result.is_err() {
        fail("Cannot scan and load local resource")
    } else {
        success("Scan and load local resource done.");
    }


    // 初始化脚本
    if engine_conf.get("engine").unwrap().get("run-init-script").unwrap().as_bool().unwrap() {
        info("Default script running...");
        load_and_exec_default_script();
        success("Run init script done.");
    } else {
        info("Skip init script running.");
    }

    // 初始流
    if engine_conf.get("engine").unwrap().get("run-init-flow").unwrap().as_bool().unwrap() {
        info("Default flow running...");
        load_and_exec_default_flow().await;
        success("Run init flow done.");
    } else {
        info("Skip init flow running.");
    }

    // 返回成功信息
    return Ok("Engine init success.".parse().unwrap());
}

// 加载当前环境信息
// 比如当前系统中的脚本，流程等信息，这些信息会被加载到数据库中
pub fn scan_load_local(mode: &str) -> std::result::Result<String, String> {
    // 这种写法虽然繁琐了点，但可以节省一小部分的内存...
    match mode {
        "script" => {
            let script_path = get_runtime_conf("script_path").unwrap();
            traverse_folder(Path::new(script_path.as_str()), "script");
        }
        "flow" => {
            let flow_path = get_runtime_conf("flow_path").unwrap();
            traverse_folder(Path::new(flow_path.as_str()), "flow");
        }
        "ext" => {
            let ext_path = get_runtime_conf("ext_path").unwrap();
            traverse_folder(Path::new(ext_path.as_str()), "ext");
        }
        _ => {
            let ext_path = get_runtime_conf("ext_path").unwrap();
            let script_path = get_runtime_conf("script_path").unwrap();
            let flow_path = get_runtime_conf("flow_path").unwrap();
            // 加载脚本信息
            traverse_folder(Path::new(script_path.as_str()), "script");
            // 加载流信息
            traverse_folder(Path::new(flow_path.as_str()), "flow");
            // 加载插件信息
            traverse_folder(Path::new(ext_path.as_str()), "ext");
        }
    }


    // 返回成功消息
    return Ok("Scan done.".to_string());
}

fn traverse_folder(folder_path: &Path, traverse_type: &str) {
    // 判断给定的路径是否存在
    let path_exist = Path::new(folder_path).is_dir();
    if !path_exist {
        warn("folder not found, ignored err and rebuilt.");
        // 不存在的话自动创建一下
        fs::create_dir(folder_path).expect("Cannot rebuild path.");
    }
    if let Ok(entries) = fs::read_dir(folder_path) {
        // 循环指定的目录
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if !get_runtime_conf("engine-mode").unwrap().eq("memory") {
                        let db_path = get_runtime_conf("db_path").unwrap();
                        // 连接到数据库（获取到的信息需要写入到数据库中）
                        let conn = Connection::open(format!("./{}/simx.db", db_path)).unwrap();
                        let table_name = format!("simx_{}", traverse_type);
                        let sql = format!("insert into {} (display_name, file_name, file_path, file_type) values (?1, ?2, ?3, ?4);", table_name);
                        let _ = conn.execute(sql.as_str(), &[
                            path.file_name().unwrap().to_str().unwrap(),
                            path.file_name().unwrap().to_str().unwrap(),
                            path.to_str().unwrap(),
                            // 通过这种方式添加的数据均为本地数据
                            "local"
                        ]);
                        conn.close().unwrap()
                    }
                    // 插件的信息会直接进入内存
                    if traverse_type.eq("ext") {
                        load_extension_by_path(path.as_path());
                    }
                    // 将数据放到 runtime 配置中
                    set_runtime_conf(format!("{}_{:?}", traverse_type, path.file_name().unwrap().to_str()).as_str(), path.to_str().unwrap());
                } else if path.is_dir() {
                    // 多级文件夹，继续遍历其中的文件和文件夹
                    traverse_folder(path.as_path(), traverse_type);
                }
            }
        }
    }
}

// 将指定路径下的插件信息加载到内存中
pub fn load_extension_by_path(path: &Path) {
    // 判断插件类型
    if path.exists() {
        if path.file_name().unwrap().to_str().unwrap().eq("extension.json") {

            // 读取 JSON 文件
            let file_path = Path::new(path);
            let data = fs::read_to_string(file_path).expect("Unable to read file");
            let mut extension: Extension = from_str(&data).expect("JSON was not well-formatted");
            extension.path = Some(file_path.parent().unwrap().to_str().unwrap().to_string());

            if !get_runtime_conf("engine-mode").unwrap().eq("memory") {
                // 循环指定的目录
                let table_name = "simx_ext".to_string();
                // 数据库地址
                let db_path = get_runtime_conf("db_path").unwrap();
                // 连接到数据库（获取到的信息需要写入到数据库中）
                let conn = Connection::open(format!("./{}/simx.db", db_path)).unwrap();
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

            // 将数据放到 runtime 配置中
            set_runtime_conf(format!("ext_{}", extension.name.as_str()).as_str(), serde_json::to_string(&extension).unwrap().as_str());
        }
    }
}

// 将指定路径下的流程信息加载到内存中
pub fn load_flow_by_path() {}

// 将指定路径下的脚本信息加载到内存中
pub fn load_script_by_path() {}