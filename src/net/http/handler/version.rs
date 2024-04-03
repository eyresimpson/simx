
// 获取当前simx版本
#[get("/version")]
pub fn handle_version_current() -> &'static str {
    return "1.0.0";
}

// 获取版本列表
#[get("/list-version")]
pub fn handle_version_list() -> &'static str {
    return "Ok";
}

// 查询最新版本
#[get("/search-new")]
pub fn handle_version_latest() -> &'static str {
    return "Ok";
}
