use rusqlite::{Connection, Result};

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

// 创建数据库连接
pub fn init_base_db_struct() -> Result<()> {
    let conn = Connection::open("./db/simx.db")?;

    conn.execute("create table if not exists simx_conf (
             id integer primary key,
             name text not null unique
         )", ())?;

    Ok(())
}
