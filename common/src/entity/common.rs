use crate::entity::exception::common::Level;
use crate::entity::flow::flow::{Flow, FlowData};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct HistoryLog{
    pub node_id: Option<String>,
    pub flow_name: Option<String>,
    pub node_name: Option<String>,
    pub level: Level,
    pub create_time: Option<String>,
    // 数据快照，根据用户需求，可以拍摄多个快照，固定第一个为开始，最后一个为结束
    // 节点开始时的快照固定为start
    // 节点结束时的快照固定为end
    // 用户定义的固定前缀为user_
    // 错误时快照固定前缀为fail_
    pub snapshot: Option<HashMap<String, FlowData>>,
    // 日志消息
    pub message: String,
}