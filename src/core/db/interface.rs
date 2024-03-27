use rusqlite::Connection;

// 创建数据库连接
pub fn init_base_db_struct() -> rusqlite::Result<()> {
    let conn = Connection::open("./db/simx.db")?;

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
