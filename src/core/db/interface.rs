use rusqlite::Connection;

// 初始化数据库
// 后续可能放到某个文件中，都写代码里太复杂了
pub fn init_base_db_struct() -> rusqlite::Result<()> {
    let conn = Connection::open("./db/simx.db")?;

    // 创建脚本表
    conn.execute("create table simx_script
                    (
                        id  integer not null
                            constraint id
                                primary key autoincrement,
                        display_name TEXT,
                        file_name    TEXT,
                        file_path    TEXT,
                        file_type    TEXT
                    );
                ", ()
    )?;

    // 创建流程表
    conn.execute("create table simx_flow
                    (
                        id  integer not null
                            constraint id
                                primary key autoincrement,
                        display_name TEXT,
                        file_name    TEXT,
                        file_path    TEXT,
                        file_type    TEXT
                    );
                ", ()
    )?;

    // 创建插件表
    conn.execute("create table simx_ext
                    (
                        id  integer not null
                            constraint id
                                primary key autoincrement,
                        display_name TEXT,
                        file_name    TEXT,
                        file_path    TEXT,
                        file_type    TEXT
                    );
                ", ()
    )?;

    Ok(())
}
