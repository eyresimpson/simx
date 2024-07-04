use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rusqlite::{Connection, params};
use tokio::task;

use crate::core::common::log::interface::{info, success};
use crate::core::flow::interface::exec_flow;
use crate::db::interface::query_data_by_id;
use crate::entity::db::SimxResultVec;
use crate::entity::net::{RequestData, SimxResponse};

#[post("/flow/exec", format = "application/json", data = "<request>")]
pub async fn handle_exec_flow_by_path(request: Json<RequestData>) -> Result<Json<SimxResponse>, Status> {
    // 处理接收到的请求数据
    let received_data = request.into_inner();
    let results =
        task::spawn_blocking(move || {
            query_data_by_id(received_data.id, "simx_flow")
        }).await.unwrap().await.unwrap();

    let results = match results {
        SimxResultVec::SimxFlow(data) => data,
        SimxResultVec::SimxScript(_) => panic!("Invalid data type"),
    };

    // 执行流程
    for result in results {
        info(format!("Exec flow {} start by http.", result.display_name).as_str());
        exec_flow(result.file_path.as_ref()).await;
        success(format!("Exec flow {} success.", result.display_name).as_str());
    }

    // 创建响应数据
    let response_data = SimxResponse {
        message: "Exec flow success.".to_string(),
        code: 200,
    };

    // 返回响应数据
    Ok(Json(response_data))
}


// 列出所有流程
#[get("/flow/list")]
pub fn handle_list_flow() -> &'static str {
    let conn = Connection::open("./db/simx.db").unwrap();
    let mut stmt = conn.prepare(
        "select * from simx_script",
    ).unwrap();
    let ret = stmt.query(()).unwrap();
    println!("{:?}", ret.as_ref());
    return "Ok";
}

// 搜索指定脚本（并非执行）
#[get("/flow/search")]
pub fn handle_search_flow() -> &'static str {
    return "Ok";
}