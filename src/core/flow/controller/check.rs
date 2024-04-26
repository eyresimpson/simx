use crate::conf::runtime::get_runtime_conf;
use crate::core::common::log::interface::{fail, warn};
use crate::core::flow::entity::standardisation::Flow;

pub fn check_require(flow: Flow) -> bool {
    // 获取要求列表
    let require = flow.env_req;
    for env in require {
        return match env.env_key.as_str() {
            "engine" => {
                // 检查引擎版本是否匹配
                check_engine(env.env_val.as_str())
            }
            _ => {
                // 提示未知的检查项
                warn("An unknown dependency was found while verifying an environment dependency.");
                true
            }
        };
    }
    // 一般不会在这里直接给拦截的
    return true;
}

fn check_engine(version: &str) -> bool {
    let current = get_runtime_conf("engine_version").unwrap();
    if current.eq(version) { true } else {
        fail("The process is stopped because the engine version check fails, check you engine version.");
        false
    }
}