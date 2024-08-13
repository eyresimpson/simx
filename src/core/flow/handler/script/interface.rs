use crate::core::script::interface::exec_script;
use crate::entity::flow::Node;

pub fn handle_script(node: Node) {
    // 调用script的执行方法
    // 要求用户在节点中配置path属性
    exec_script(node.attr.get("path").unwrap().as_str().as_ref());
}