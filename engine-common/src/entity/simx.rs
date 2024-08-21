use serde_derive::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxFlow {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimxScript {
    pub id: i32,
    pub display_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_type: String,
}

pub struct SimxThreadFunctions {
    pub exec_script: fn(&Path),
    pub exec_flow: fn(&Path),
}

pub struct SimxThreadSenderStringData {
    pub function: String,
    pub data: String,
}