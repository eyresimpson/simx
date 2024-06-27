use rusqlite::Connection;
use crate::core::flow::interface::exec_flow;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
struct RequestData {
    field1: String,
    field2: i32,
}

#[derive(Serialize, Deserialize)]
struct ResponseData {
    message: String,
    code: i32,
}

#[post("/flow/exec-by-path", format = "application/json", data = "<request>")]
pub async fn handle_exec_flow_by_path(request: Json<RequestData>) -> Result<Json<ResponseData>, Status> {
    // 处理接收到的请求数据
    let received_data = request.into_inner();
    println!("Received field1: {}, field2: {}", received_data.field1, received_data.field2);
    exec_flow("flow/init/hello.flow".as_ref()).await;
    // 创建响应数据
    let response_data = ResponseData {
        message: format!("Received field1: {} and field2: {}", received_data.field1, received_data.field2),
        code: 200,
    };

    // 返回响应数据
    Ok(Json(response_data))
}


// 列出所有脚本
#[get("/list-script")]
pub fn handle_list_flow() -> &'static str {
    let conn = Connection::open("./db/simx.db").unwrap();
    let mut stmt = conn.prepare(
        "select * from simx_script",
    ).unwrap();
    let ret = stmt.query(()).unwrap();
    println!("===> {:?}", ret.as_ref().unwrap());
    return "Ok";
}

// 搜索指定脚本
#[get("/search-flow")]
pub fn handle_search_script() -> &'static str {
    return "Ok";
}

//