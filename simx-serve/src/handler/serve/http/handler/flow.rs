//
// #[post("/flow/exec", format = "application/json", data = "<request>")]
// pub async fn handle_exec_flow_by_path(request: Json<ExecFlowRequestData>) -> Result<Json<SimxResponse>, Status> {
//     // 处理接收到的请求数据
//     let received_data = request.into_inner();
// 
//     // 执行流程
//     info("Exec flow start by http.".to_string().as_str());
//     exec_flow(received_data.path.unwrap().as_ref()).await;
// 
//     get_flow_info("");
// 
//     // 创建响应数据
//     let response_data = SimxResponse {
//         message: "Exec flow success.".to_string(),
//         code: 200,
//         data: "".to_string(),
//     };
// 
//     // 返回响应数据
//     Ok(Json(response_data))
// }
// 
// 
// // 列出所有流程
// #[post("/flow/list")]
// pub async fn handle_list_flow() -> Result<Json<SimxResponse>, Status> {
//     let flows = get_all_flow_info();
//     let result_data = SimxResponse {
//         message: "List flow success.".to_string(),
//         code: 200,
//         data: serde_json::to_string_pretty(&flows).unwrap(),
//     };
//     Ok(Json(result_data))
// }
// 
// // 搜索指定流程（并非执行）
// #[get("/flow/search")]
// pub fn handle_search_flow() -> &'static str {
//     return "Ok";
// }