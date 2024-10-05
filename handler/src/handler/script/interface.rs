use engine_common::entity::error::NodeError;
use engine_common::entity::flow::Node;
use engine_common::thread::script::exec_script;

pub fn handle_script(node: Node) -> Result<(), NodeError> {
    // 调用script的执行方法
    // 要求用户在节点中配置path属性
    let ret = exec_script(node.attr.get("path").unwrap().to_string());
    if ret.is_err() {
        return Err(NodeError::ScriptExecError(ret.err().unwrap()));
    }
    Ok(())
}