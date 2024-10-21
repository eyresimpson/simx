use crate::entity::common::HistoryLog;
use crate::entity::exception::common::Level;

// 节点日志
pub fn history(log: HistoryLog) {
    match log.level {
        Level::NodeFail => {}
        Level::NodeWarn => {}
        Level::NodeInfo => {}
        Level::NodeDebug => {}
        Level::EngineFail => {}
        Level::EngineWarn => {}
        Level::EngineInfo => {}
        Level::EngineDebug => {}
        Level::ScriptInfo => {}
        Level::ScriptWarn => {}
        Level::ScriptFail => {}
        Level::ScriptDebug => {}
        Level::FlowInfo => {}
        Level::FlowWarn => {}
        Level::FlowFail => {}
        Level::FlowDebug => {}
        Level::None => {}
    }
}
