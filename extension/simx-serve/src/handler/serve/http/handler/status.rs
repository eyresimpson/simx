use rocket::get;

// 获取当前simx版本
#[get("/common/version/current")]
pub fn handle_version_current() -> String {
    "version".to_string()
}


// 获取版本列表
#[get("/common/version/list-version")]
pub fn handle_version_list() -> &'static str {
    "Ok"
}

// 查询最新版本
#[get("/common/version/new")]
pub fn handle_version_latest() -> &'static str {
    "Ok"
}
