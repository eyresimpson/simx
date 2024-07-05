use rocket::http::Status;
use rocket::serde::json::Json;
use tokio::task;

use crate::core::common::log::interface::{info, success};
use crate::core::script::interface::exec_script;
use crate::db::interface::query_data_by_id;
use crate::entity::db::{SimxResultVec, SimxScript};
use crate::entity::net::{ExecScriptRequestData, SimxResponse};

// 执行指定脚本
#[post("/script/exec", format = "application/json", data = "<request>")]
pub async fn handle_exec_script(request: Json<ExecScriptRequestData>) -> Result<Json<SimxResponse>, Status> {
    // 处理接收到的请求数据
    let received_data = request.into_inner();
    // 在阻塞任务中查询数据
    let results =
        task::spawn_blocking(move || {
            query_data_by_id(received_data.id, "simx_script")
        }).await.unwrap().await.unwrap();

    let results = match results {
        SimxResultVec::SimxScript(data) => data,
        SimxResultVec::SimxFlow(_) => panic!("Invalid data type"),
    };

    // 执行流程
    for result in results {
        info(format!("Exec script {} [{}] start by http.", result.display_name, result.id).as_str());
        exec_script(result.file_path.as_ref());
        success(format!("Exec script {} [{}] success.", result.display_name, result.id).as_str());
    }
    // 创建响应数据
    let response_data = SimxResponse {
        message: "Exec script success.".to_string(),
        code: 200,
        data: "".to_string(),
    };
    Ok(Json(response_data))
}

// 列出所有脚本
#[post("/script/list")]
pub async fn handle_list_script() -> Result<Json<Vec<SimxScript>>, Status> {
    let def = "*".to_string();
    let results =
        task::spawn_blocking(move || {
            query_data_by_id(def, "simx_script")
        }).await.unwrap().await.unwrap();

    let results = match results {
        SimxResultVec::SimxScript(data) => data,
        // 一般不可能进这个分支
        SimxResultVec::SimxFlow(_) => panic!("Invalid data type"),
    };
    Ok(Json(results))
}

// 搜索指定脚本
#[get("/script/search")]
pub fn handle_search_script() -> &'static str {
    return "Ok";
}
