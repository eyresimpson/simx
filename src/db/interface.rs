use rusqlite::{Connection, params, Rows};

use crate::conf::runtime::get_runtime_conf;
use crate::entity::db::{SimxFlow, SimxResultVec, SimxScript};

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
                ", (),
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
                ", (),
    )?;

    // 创建插件表
    conn.execute("create table simx_ext
                    (
                        id  integer not null
                            constraint id
                                primary key autoincrement,
                        display_name TEXT,
                        file_name    TEXT,
                        ext_path    TEXT,
                        ext_type    TEXT,
                        version    TEXT,
                        file_type    TEXT
                    );
                ", (),
    )?;

    // // 创建日志表
    // conn.execute("create table simx_log
    //                 (
    //                     id  integer not null
    //                         constraint id
    //                             primary key autoincrement,
    //                     log_datetime TEXT,
    //                     log_type    TEXT,
    //                     log_message    TEXT,
    //                 );
    //             ", (),
    // )?;

    Ok(())
}

// 根据id查询数据库
pub async fn query_data_by_id(id: String, table_name: &str) -> Result<SimxResultVec, rusqlite::Error> {
    // 从配置中获取数据库路径
    let db_path = get_runtime_conf("db_path").unwrap();
    // 链接到数据库
    let conn = Connection::open(format!("{}/simx.db", db_path))?;
    let sql: String;
    let mut stmt;
    let mut rows: Rows;
    if id != "*" {
        sql = format!("SELECT * FROM {} WHERE id = (?1)", table_name);
        stmt = conn.prepare(sql.as_str())?;
        rows = stmt.query(params![id]).unwrap()
    } else {
        sql = format!("SELECT * FROM {}", table_name);
        stmt = conn.prepare(sql.as_str())?;
        rows = stmt.query(params![]).unwrap()
    }

    let mut flow_results = Vec::new();
    let mut script_results = Vec::new();
    // 遍历行
    while let Some(row) = rows.next().unwrap() {
        if table_name.eq("simx_flow") {
            let simx_flow = SimxFlow {
                id: row.get(0)?,
                display_name: row.get(1)?,
                file_name: row.get(2)?,
                file_path: row.get(3)?,
                file_type: row.get(4)?,
            };
            flow_results.push(simx_flow);
        } else {
            let simx_script = SimxScript {
                id: row.get(0)?,
                display_name: row.get(1)?,
                file_name: row.get(2)?,
                file_path: row.get(3)?,
                file_type: row.get(4)?,
            };
            script_results.push(simx_script);
        }
    }

    if table_name.eq("simx_flow") {
        Ok(SimxResultVec::SimxFlow(flow_results))
    } else {
        Ok(SimxResultVec::SimxScript(script_results))
    }
}

// 根据id列表查询数据库
