use rocket::get;

// 搜索指定脚本
#[get("/script/search")]
pub fn handle_search_script() -> &'static str {
    return "Ok";
}
