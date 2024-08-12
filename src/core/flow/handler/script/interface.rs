use crate::core::script::interface::exec_script;
use crate::entity::flow::Node;

pub fn handle_script(node: Node) {
    exec_script(node.attr.get("path").unwrap().as_str().as_ref());
}