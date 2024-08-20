use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};


// 列出所有脚本
// #[post("/script/list", format = "application/json")]
// pub async fn handle_list_script() -> Result<Json<SimxResponse>, Status> {
//     let scripts = get_all_script_info();
//     let result_data = SimxResponse {
//         message: "List scripts success.".to_string(),
//         code: 200,
//         data: serde_json::to_string_pretty(&scripts).unwrap(),
//     };
//     Ok(Json(result_data))
// }

// 执行指定脚本
// #[post("/script/exec", format = "application/json")]
// pub async fn handle_exec_script() -> Result<Json<SimxResponse>, Status> {
//     let scripts = get_all_script_info();
//     get_script_info("");
//     let result_data = SimxResponse {
//         message: "List scripts success.".to_string(),
//         code: 200,
//         data: serde_json::to_string_pretty(&scripts).unwrap(),
//     };
//     Ok(Json(result_data))
// }


// 搜索指定脚本
#[get("/script/search")]
pub fn handle_search_script() -> &'static str {
    return "Ok";
}
