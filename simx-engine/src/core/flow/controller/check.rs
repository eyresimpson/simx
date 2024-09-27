use crate::core::environment::interface::check;
use engine_common::entity::flow::Flow;

pub fn check_require(flow: Flow) -> bool {
    // 获取要求列表
    let require = flow.requirements;
    check(require);
    return true;
}