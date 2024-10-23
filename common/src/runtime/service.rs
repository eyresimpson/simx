use crate::entity::services::Service;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    // 服务运行时
    static ref FLOW_HISTORY: Mutex<HashMap<String, Service>> = Mutex::new(HashMap::new());
}