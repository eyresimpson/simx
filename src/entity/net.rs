use serde::Serialize;
use serde_derive::Deserialize;

// 请求执行脚本
#[derive(Serialize, Deserialize)]
pub struct ExecScriptRequestData {
    pub(crate) id: String,
    pub url: String
}

// 发起流时使用的请求数据结构
#[derive(Serialize, Deserialize)]
pub struct ExecFlowRequestData {
    pub id: String,
    pub url: String
}

#[derive(Serialize, Deserialize)]
pub struct SimxResponse {
    // 编码，200 代表成功
    pub(crate) code: i32,
    // 消息
    pub(crate) message: String,
    // 数据，建议为Json字符串
    pub data: String,
}

// pub struct FlowListResponse {
//     pub(crate) code: i32,
//     pub(crate) message: String,
//     pub(crate) data: Vec<SimxFlow>,
// }

// pub struct ScriptListResponse {
//     pub(crate) code: i32,
//     pub(crate) message: String,
//     pub(crate) data: Vec<SimxFlow>,
// }
