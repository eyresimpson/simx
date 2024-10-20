use crate::entity::flow::flow::Flow;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxFlow {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
    // 流对象，加载时机是执行流前
    pub flow: Option<Flow>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxScript {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
}


pub struct SimxThreadSenderStringData {
    pub function: String,
    pub data: String,
}