use engine_common::entity::error::NodeError;
use engine_common::entity::flow::Node;
use engine_common::thread::script::exec_script;
use engine_common::tools::format::u8_to_str;

pub fn handle_script(node: Node) -> Result<(), NodeError> {
    match node.attr.get("path") {
        Some(path) => {
            let path = u8_to_str(path.clone());
            match exec_script(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(NodeError::ScriptExecError(e))
            }
        }
        None => Err(NodeError::ParamNotFound("path".to_string())),
    }
}