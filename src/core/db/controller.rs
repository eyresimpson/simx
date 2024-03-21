use rusqlite::{Connection, Result};

use crate::core::db::interface::init_base_db_struct;
use crate::tools::log::shell::{err, warn};

// 此方法用于初始化数据库（如果需要的话）
pub fn db_init() -> Result<()> {
    let conn = Connection::open("./db/simx.db")?;
    let r = conn.execute("select 1 from simx_conf", ());
    if r.is_err() {
        warn("cannot find table, will init it.");
        let ir = init_base_db_struct();
        if ir.is_err() {
            err("cannot init system db struct, engine init failed!");
        }
    }
    Ok(())
}

// pub fn db_reload() {}