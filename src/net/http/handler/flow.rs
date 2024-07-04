use rocket::http::Status;
use rocket::serde::{Deserialize, json::Json, Serialize};
use rusqlite::{Connection, params};
use tokio::task;

use crate::conf::runtime::get_runtime_conf;
use crate::core::common::log::interface::{info, success};
use crate::core::flow::interface::exec_flow;
use crate::entity::db::SimxFlow;

#[derive(Serialize, Deserialize)]
// 发起流时使用的请求数据结构
pub struct RequestData {
    flow_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    message: String,
    code: i32,
}

async fn query_data(flow_id: String) -> Result<Vec<SimxFlow>, rusqlite::Error> {
    // 从配置中获取数据库路径
    let db_path = get_runtime_conf("db_path").unwrap();
    // 链接到数据库
    let conn = Connection::open(db_path).unwrap();
    // 执行sql
    // TODO： SQL可能需要优化一下
    let mut stmt = conn.prepare("SELECT * FROM simx_flow WHERE id = (?1)")?;
    // 拆解行
    let mut rows = stmt.query(params![flow_id])?;
    let mut results = Vec::new();
    // 遍历行
    while let Some(row) = rows.next()? {
        let simx_flow = SimxFlow {
            id: row.get(0)?,
            display_name: row.get(1)?,
            file_name: row.get(2)?,
            file_path: row.get(3)?,
            file_type: row.get(4)?,
        };
        results.push(simx_flow);
    }

    Ok(results)
}

#[post("/flow/exec", format = "application/json", data = "<request>")]
pub async fn handle_exec_flow_by_path(request: Json<RequestData>) -> Result<Json<ResponseData>, Status> {
    // 处理接收到的请求数据
    let received_data = request.into_inner();
    // 在阻塞任务中查询数据
    let results = task::spawn_blocking(move || {
        query_data(received_data.flow_id)
    }).await.unwrap().await.unwrap();

    // 执行流程
    for result in results {
        info(format!("Exec flow {} start.", result.display_name).as_str());
        exec_flow(result.file_path.as_ref()).await;
        success(format!("Exec flow {} success.", result.display_name).as_str());
    }

    // 创建响应数据
    let response_data = ResponseData {
        message: "Exec Flow".to_string(),
        code: 200,
    };

    // 返回响应数据
    Ok(Json(response_data))
}


// 列出所有脚本
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