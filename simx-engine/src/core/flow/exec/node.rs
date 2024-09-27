use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::debug;
use engine_handler::handler::interface::root_handler;

// Node 调度
// Node 需要对应的Handler执行
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub async fn exec_node(node: Node, data: &mut FlowData) -> Result<(), String> {
    debug("[ Node Exec Start ]");

    // 打印提示信息
    // debug(format!("Node Input -> handler: {}, data: {:?}, args: {:?}", node.handler, data, node.attr).as_str());

    // 解析表达式
    // resolve_said_expression("test{a1}", "str");

    let ret = root_handler(node, data).await;

    debug("[ Node Exec End ]");
    ret
}