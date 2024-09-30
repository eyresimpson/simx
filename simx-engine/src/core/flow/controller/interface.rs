use crate::core::environment::interface::check;
use crate::core::flow::dispatch::interface::dispatch_flow;
use engine_common::entity::flow::Environment;
use engine_common::logger::interface::{success, warn};
use std::path::Path;

// 这个东西其实就是Json（目前来说），后续可能会进行一些加密的操作
pub async fn exec_fl_flow(path: &Path) {
    // 调度执行
    match dispatch_flow(path).await {
        Ok(_) => {
            success(format!("Flow {:?} execution completed normally", path).as_str());
            // 正常执行结束，退出
            return;
        }
        Err(e) => {
            warn(format!("Flow {:?} execution ended abnormally, more info: {:?}", path, e).as_str());
            // 暂时不处理，退出
            return;
        }
    }
}

pub fn check_require(require: Vec<Environment>) -> Result<(), String> {
    check(require)
}