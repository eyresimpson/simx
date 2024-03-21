use rusqlite::Connection;

// 创建数据库连接
pub fn init_base_db_struct() -> rusqlite::Result<()> {
    let conn = Connection::open("./db/simx.db")?;

    conn.execute("create table if not exists simx_conf (
             id integer primary key,
             name text not null unique
         )", ())?;

    Ok(())
}

// 根据名称查询环境配置
// pub fn queryEnvByName() {}
//
// // 根据ID查询环境配置
// pub fn queryEnvById() {}
//
// // 查询所有的环境配置
// pub fn queryAllEnv() {}

