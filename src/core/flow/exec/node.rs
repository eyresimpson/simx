use crate::core::flow::expression::interface::resolve_said_expression;
use crate::core::flow::handler::interface::handler;
use crate::entity::flow::{FlowData, Node};
use crate::tools::log::interface::{debug, fail};

// Node 调度
// Node 需要对应的Handler执行
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub async fn exec_node(node: Node, data: &mut FlowData) -> Result<(), String> {
    debug("[ Node Exec Start ]");

    // 打印提示信息
    debug(format!("Node Input -> handler: {}, data: {:?}, args: {:?}", node.handler, data, node.attr).as_str());

    // 解析表达式
    resolve_said_expression("test{a1}", "str");

    // 执行 handler
    // 判断该handel是否为不允许失败的操作，如果是，流程会强制结束，否则会忽略错误并继续
    if node.attr.contains_key("vital") && node.attr.get("vital").unwrap() == "true" {
        handler(node, data).await.unwrap();
    } else {
        let res = handler(node, data).await;
        res.unwrap_or_else(|err| {
            fail(format!("[SKIP] Node Exec Error -> {}", err).as_str());
        });
    }

    debug(format!("Node Output -> data: {:?}", data).as_str());
    debug("[ Node Exec End ]");
    Ok(())
}