use crate::entity::common::HistoryLog;
use crate::entity::flow::flow::HistoryState;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    // 流运转历史日志（这个日志一定小心，很容易把内存撑满）
    // key为流的flow id，val是该流的日志数组
    static ref FLOW_HISTORY: Mutex<HashMap<String, HashMap<HistoryState, HistoryLog>>> = Mutex::new(HashMap::new());

}

pub fn log_history(flow_id: String, state: HistoryState, value: HistoryLog) {
    let mut data = FLOW_HISTORY.lock().unwrap();
    let mut map: HashMap<HistoryState, HistoryLog> = HashMap::new();
    map.insert(state, value);
    data.insert(flow_id, map);
}

pub fn history_persistent(flow_id: String) {
    let mut data = FLOW_HISTORY.lock().unwrap();
    let history = data.get(flow_id.as_str()).expect("flow_id not found");
}