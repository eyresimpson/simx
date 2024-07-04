use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct FlowRequest {
    path: String,
}

// 请求执行脚本
#[derive(Serialize, Deserialize)]
pub struct ExecScriptRequestData {
    pub(crate) id: String,
}

// 发起流时使用的请求数据结构
#[derive(Serialize, Deserialize)]
pub struct RequestData {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SimxResponse {
    // {"code":200,"message":"导入成功","data":{"succCount":1,"skipCount":0}}
    pub(crate) code: i32,
    pub(crate) message: String,
    // pub(crate) data: Box<dyn Any>
}