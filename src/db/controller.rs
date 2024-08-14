use rusqlite::{Connection, Result};

use crate::core::common::log::interface::fail;
use crate::db::interface::init_base_db_struct;
use crate::entity::config::engine::get_engine_config;

// 此方法用于初始化数据库（如果需要的话）
pub fn db_init() -> Result<()> {
    let db_path = get_engine_config().engine.db_path;
    let conn = Connection::open(format!("./{}/simx.db", db_path)).unwrap();
    let conf = get_engine_config().engine;
    let auto_refresh = conf.auto_refresh_local_data;
    if auto_refresh {
        // 删除指定的数据表（方法太笨，后续改进）
        let tables = ["simx_script", "simx_flow", "simx_ext"];

        for table in &tables {
            let query = format!("DROP TABLE IF EXISTS {}", table);
            conn.execute(&query, [])?;
        }
    }
    let exists = conn.query_row(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
        &[&"simx_script"],
        |_| Ok(()),
    );

    if exists.is_err() {
        // warn("cannot find table, will init it.");
        // 初始化数据表结构
        // 后续这部分内容应该会移动到sql文件中，而不是内置在程序里，这样可能比较浪费内存空间，目前表还少，先这样用着
        let ir = init_base_db_struct();
        if ir.is_err() {
            fail("cannot init system db struct, engine init failed!");
        }
        // 加载数据到数据库
        // 只有数据库文件不存在的时候才会去重新加载本地内容
        // let ret = scan_load_local();
        // if ret.is_err() {
        //     warn("Cannot load local Data.");
        // }
    }

    Ok(())
}

