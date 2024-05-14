use rusqlite::Connection;

// 执行指定脚本
#[get("/exec-script")]
pub fn handle_exec_script() -> &'static str {
    return "Ok";
}

// 列出所有脚本
#[get("/list-script")]
pub fn handle_list_script() -> &'static str {
    let conn = Connection::open("./db/simx.db").unwrap();
    let mut stmt = conn.prepare(
        "select * from simx_script",
    ).unwrap();
    let ret = stmt.query(()).unwrap();
    println!("===> {:?}", ret.as_ref().unwrap());
    return "Ok";
}

// 搜索指定脚本
#[get("/search-script")]
pub fn handle_search_script() -> &'static str {
    return "Ok";
}

//