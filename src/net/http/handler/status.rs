use crate::conf::runtime::{get_runtime_conf};

// 获取当前simx版本
// TODO 改造成restful风格（Json）
#[get("/common/version/current")]
pub fn handle_version_current() -> String {
    let version = format!("engine current version: {:?}\nsupport api version: {:?}", get_runtime_conf("engine_version").unwrap(), get_runtime_conf("support_api_version").unwrap());
    return version;
}


// 获取版本列表
#[get("/common/version/list-version")]
pub fn handle_version_list() -> &'static str {
    return "Ok";
}

// 查询最新版本
#[get("/common/version/new")]
pub fn handle_version_latest() -> &'static str {
    return "Ok";
}

// 升级到指定版本

// 获取引擎运行状况