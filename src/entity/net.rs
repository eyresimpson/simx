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
    pub(crate) code: i32,
    pub(crate) message: String,
}