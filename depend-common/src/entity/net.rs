use serde::Serialize;
use serde_derive::Deserialize;

// 请求执行脚本
#[derive(Serialize, Deserialize)]
pub struct ExecScriptRequestData {
    pub id: String,
    pub url: String,
}

// 发起流时使用的请求数据结构
#[derive(Serialize, Deserialize)]
pub struct ExecFlowRequestData {
    pub id: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SimxResponse {
    // 编码，200 代表成功
    pub code: i32,
    // 消息
    pub message: String,
    // 数据，建议为Json字符串
    pub data: String,
}

// pub struct FlowListResponse {
//     pub code: i32,
//     pub message: String,
//     pub data: Vec<SimxFlow>,
// }

// pub struct ScriptListResponse {
//     pub code: i32,
//     pub message: String,
//     pub data: Vec<SimxFlow>,
// }
