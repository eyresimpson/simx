use crate::core::common::log::interface::debug;
use crate::core::flow::expression::interface::resolve_said_expression;
use crate::core::flow::handler::interface::handler;
use crate::entity::flow::{FlowData, Node};

// Node 调度
// Node 需要对应的Handler执行
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub async fn exec_node(node: Node, data: &mut FlowData) {
    debug("[ Node Exec Start ]");

    // 打印提示信息
    debug(format!("Node Input -> handler: {}, data: {:?}, args: {:?}", node.handler, data, node.attr).as_str());

    // 解析表达式
    resolve_said_expression("test{a1}", "str");

    // 执行 handler
    handler(node, data).await;

    debug(format!("Node Output -> data: {:?}", data).as_str());
    debug("[ Node Exec End ]");
}