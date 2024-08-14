use rocket::http::Status;
use rocket::serde::json::Json;

use crate::entity::net::{ExecScriptRequestData, SimxResponse};

// 执行指定脚本
#[post("/script/exec", format = "application/json", data = "<request>")]
pub async fn handle_exec_script(request: Json<ExecScriptRequestData>) -> Result<Json<SimxResponse>, Status> {
    // 处理接收到的请求数据

    // 创建响应数据
    let response_data = SimxResponse {
        message: "Exec script success.".to_string(),
        code: 200,
        data: "".to_string(),
    };
    Ok(Json(response_data))
}


// 搜索指定脚本
#[get("/script/search")]
pub fn handle_search_script() -> &'static str {
    return "Ok";
}
