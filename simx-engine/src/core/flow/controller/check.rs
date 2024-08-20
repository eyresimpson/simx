use crate::core::environment::interface::check;
use crate::entity::flow::Flow;

pub fn check_require(flow: Flow) -> bool {
    // 获取要求列表
    let require = flow.requirements;
    check(require);
    // for environment in require {
    //     let ret = match environment.env_key.as_str() {
    //         "engine" => {
    //             // 检查引擎版本是否匹配
    //             check_engine(environment.env_val.as_str())
    //         }
    //         "java" => {
    //
    //             // 检查本地java版本
    //             // check(Vec::new().append(""));
    //             true
    //         }
    //         _ => {
    //             // 提示未知的检查项
    //             warn("An unknown dependency was found while verifying an environment dependency.");
    //             // 未知的依赖也会触发流停止
    //             false
    //         }
    //     };
    //     // 发现失败就退出
    //     if !ret { return false; }
    // }
    // 一般不会在这里直接给拦截的
    return true;
}

// fn check_engine(version: &str) -> bool {
//     let current = get_runtime_conf("engine_version").unwrap();
//     if current.eq(version) {
//         success("Flow engine version check success.");
//         true
//     } else {
//         fail("The process is stopped because the engine version check fails, check you engine version.");
//         false
//     }
// }
