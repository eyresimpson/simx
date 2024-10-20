use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum Level {
    // 节点错误
    NodeFail,
    // 节点警告
    NodeWarn,
    // 节点信息
    NodeInfo,
    // 节点调试
    NodeDebug,
    // 引擎错误
    EngineFail,
    // 引擎警告
    EngineWarn,
    // 引擎信息
    EngineInfo,
    // 引擎调试
    EngineDebug,
    // 脚本信息
    ScriptInfo,
    // 脚本警告
    ScriptWarn,
    // 脚本错误
    ScriptFail,
    // 脚本调试
    ScriptDebug,
    // 流信息（特指调度器）
    FlowInfo,
    // 流警告（特指调度器）
    FlowWarn,
    // 流错误（特指调度器）
    FlowFail,
    // 流调试（特指调度器）
    FlowDebug,
    // 无状态（默认）
    #[default]
    None,
}