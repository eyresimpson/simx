use std::fs;
use std::path::Path;

use rusqlite::{Connection, Result};

use crate::core::db::interface::init_base_db_struct;
use crate::tools::log::shell::{err, warn};

// 此方法用于初始化数据库（如果需要的话）
pub fn db_init() -> Result<()> {
    let conn = Connection::open("./db/simx.db")?;
    let exists = conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
        &[&"simx_script"],
        |_| Ok(()),
    );
    // let r = conn.execute("select 1 from simx_script", ());
    if exists.is_err() {
        warn("cannot find table, will init it.");
        // warn(r.err().unwrap().to_string().as_str());
        // 初始化数据表结构
        // 后续这部分内容应该会移动到sql文件中，而不是内置在程序里，这样可能比较浪费内存空间，目前表还少，先这样用着
        let ir = init_base_db_struct();
        if ir.is_err() {
            err("cannot init system db struct, engine init failed!");
        }
        // 加载数据到数据库
        // 只有数据库文件不存在的时候才会去重新加载本地内容
        // TODO：后续会允许手动重新加载配置，或者考虑主动监视文件夹改动
        let ret = scan_load_local();
        if ret.is_err() {
            warn("Cannot load local Data.");
        }
    }

    Ok(())
}

// 加载当前环境信息
// 比如当前系统中的脚本，流程等信息，这些信息会被加载到数据库中
pub fn scan_load_local() -> std::result::Result<String, String> {
    // 加载脚本信息
    traverse_folder(Path::new("script"));
    // 加载流信息
    traverse_folder(Path::new("flow"));
    // 加载插件信息
    traverse_folder(Path::new("ext"));
    // 返回成功消息
    return Ok("Scan done.".to_string());
}

fn traverse_folder(folder_path: &Path) {
    // 连接到数据库（获取到的信息需要写入到数据库中）
    let conn = Connection::open("./db/simx.db").unwrap();
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
                    let _ = conn.execute("insert into simx_script (display_name, file_name, file_path, file_type) \
                        values (?1, ?2, ?3, ?4);", &[
                        path.file_name().unwrap().to_str().unwrap(),
                        path.file_name().unwrap().to_str().unwrap(),
                        path.to_str().unwrap(),
                        folder_path.to_str().unwrap()
                    ]);
                } else if path.is_dir() {
                    // 多级文件夹，继续遍历其中的文件和文件夹
                    traverse_folder(path.as_path());
                }
            }
        }
    }
    conn.close().unwrap()
}