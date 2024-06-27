use std::any::Any;
use serde::{Serialize, Serializer};
use serde_derive::{Deserialize};

//
#[derive(Deserialize)]
struct FlowRequest {
    path: String,
}

struct SimxResponse {
    // {"code":200,"message":"导入成功","data":{"succCount":1,"skipCount":0}}
    code: i32,
    message: String,
    data: Box<dyn Any>
}