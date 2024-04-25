use crate::core::common::log::shell::info;
use crate::core::flow::entity::standardisation::{Data, Node};
use crate::core::flow::expression::interface::resolve_said_expression;
use crate::core::flow::handler::interface::handler;

// Node 调度
// Node 需要对应的Handler执行
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub async fn exec_node(node: Node, data: &mut Data) {
    info("[ Node Exec Start ]");

    // 打印提示信息
    info(format!("Node Input -> handler: {}, data: {:?}, args: {:?}", node.handler, data, node.attr).as_str());

    // 解析表达式
    resolve_said_expression("test{a1}", "str");

    // 执行 handler
    handler(node, data).await;

    info(format!("Node Output -> data: {:?}", data).as_str());
    info("[ Node Exec End ]");
}