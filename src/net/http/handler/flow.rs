use rocket::http::Status;
use rocket::serde::json::Json;

use crate::conf::runtime::get_runtime_confs;
use crate::core::common::log::interface::info;
use crate::core::flow::interface::exec_flow;
use crate::entity::net::{ExecFlowRequestData, SimxResponse};

#[post("/flow/exec", format = "application/json", data = "<request>")]
pub async fn handle_exec_flow_by_path(request: Json<ExecFlowRequestData>) -> Result<Json<SimxResponse>, Status> {
    // 处理接收到的请求数据
    let received_data = request.into_inner();

    // 执行流程
    info("Exec flow start by http.".to_string().as_str());
    exec_flow(received_data.path.unwrap().as_ref()).await;

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
    get_runtime_confs("flow_");
    // let def = "*".to_string();
    // let results =
    //     task::spawn_blocking(move || {
    //         query_data_by_id(def, "simx_flow")
    //     }).await.unwrap().await.unwrap();
    // 
    // let results = match results {
    //     SimxResultVec::SimxFlow(data) => data,
    //     // 一般不可能进这个分支
    //     SimxResultVec::SimxScript(_) => panic!("Invalid data type"),
    // };
    // let result_data = SimxResponse {
    //     message: "List flow success.".to_string(),
    //     code: 200,
    //     // data: serde_json::to_string(&results).unwrap()
    //     data: serde_json::to_string_pretty(&results).unwrap(),
    // };
    let result_data = SimxResponse {
        message: "List flow success.".to_string(),
        code: 200,
        data: "".to_string(),
    };
    Ok(Json(result_data))
}

// 搜索指定流程（并非执行）
#[get("/flow/search")]
pub fn handle_search_flow() -> &'static str {
    return "Ok";
}