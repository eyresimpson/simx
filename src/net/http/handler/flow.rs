use rocket::http::Status;
use rocket::serde::json::Json;
use tokio::task;

use crate::core::common::log::interface::{info, success};
use crate::core::flow::interface::exec_flow;
use crate::db::interface::query_data_by_id;
use crate::entity::db::SimxResultVec;
use crate::entity::net::{ExecFlowRequestData, SimxResponse};

#[post("/flow/exec", format = "application/json", data = "<request>")]
pub async fn handle_exec_flow_by_path(request: Json<ExecFlowRequestData>) -> Result<Json<SimxResponse>, Status> {
    // 处理接收到的请求数据
    let received_data = request.into_inner();
    let results =
        task::spawn_blocking(move || {
            query_data_by_id(received_data.id, "simx_flow")
        }).await.unwrap().await.unwrap();

    let results = match results {
        SimxResultVec::SimxFlow(data) => data,
        // 一般不可能进这个分支
        SimxResultVec::SimxScript(_) => panic!("Invalid data type"),
    };

    // 执行流程
    for result in results {
        info(format!("Exec flow {} [{}] start by http.", result.display_name, result.id).as_str());
        exec_flow(result.file_path.as_ref()).await;
        success(format!("Exec flow {} [{}] success.", result.display_name, result.id).as_str());
    }

    // 创建响应数据
    let response_data = SimxResponse {
        message: "Exec flow success.".to_string(),
        code: 200,
        data: "".to_string(),
    };

    // 返回响应数据
    Ok(Json(response_data))
}


// 列出所有流程
#[post("/flow/list")]
pub async fn handle_list_flow() -> Result<Json<SimxResponse>, Status> {
    let def = "*".to_string();
    let results =
        task::spawn_blocking(move || {
            query_data_by_id(def, "simx_flow")
        }).await.unwrap().await.unwrap();

    let results = match results {
        SimxResultVec::SimxFlow(data) => data,
        // 一般不可能进这个分支
        SimxResultVec::SimxScript(_) => panic!("Invalid data type"),
    };
    let result_data = SimxResponse {
        message: "List flow success.".to_string(),
        code: 200,
        // data: serde_json::to_string(&results).unwrap()
        data: serde_json::to_string_pretty(&results).unwrap(),
    };
    Ok(Json(result_data))
}

// 搜索指定流程（并非执行）
#[get("/flow/search")]
pub fn handle_search_flow() -> &'static str {
    return "Ok";
}