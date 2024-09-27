use crate::core::environment::interface::check;
use crate::core::flow::dispatch::interface::dispatch_flow;
use engine_common::entity::flow::Environment;
use std::path::Path;

// 这个东西其实就是Json（目前来说），后续可能会进行一些加密的操作
pub async fn exec_fl_flow(path: &Path) {
    dispatch_flow(path).await;
}

pub fn check_require(require: Vec<Environment>) -> Result<bool, String> {
    check(require)
}