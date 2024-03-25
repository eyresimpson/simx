// 执行指定脚本
#[get("/exec-script")]
pub fn handle_exec_script() -> &'static str {
    return "Ok";
}

// 列出所有脚本
#[get("/list-script")]
pub fn handle_list_script() -> &'static str {
    return "Ok";
}

// 搜索指定脚本
#[get("/search-script")]
pub fn handle_search_script() -> &'static str {
    return "Ok";
}
