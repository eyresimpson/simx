use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::debug;
use engine_handler::handler::interface::handler;

// Node 调度
// Node 需要对应的Handler执行
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub fn exec_node(node: Node, data: &mut FlowData) {
    if node.attr.contains_key("node_name") {
        debug(format!("[ Node ( {} ) Exec Start] ", node.attr.get("node_name").unwrap()).as_str());
    } else {
        debug("[ Node Exec Start ]");
    }


    // 打印提示信息
    // debug(format!("Node Input -> handler: {}, data: {:?}, args: {:?}", node.handler, data, node.attr).as_str());

    // 解析表达式
    // resolve_said_expression("test{a1}", "str");

    // 执行 handler
    // 判断该handel是否为不允许失败的操作，如果是，流程会强制结束，否则会忽略错误并继续
    // if node.attr.contains_key("vital") && node.attr.get("vital").unwrap() == "true" {
    //     handler(node, data).await.expect("TODO: panic message");
    // } else {
    handler(node, data).unwrap();
    // }

    // debug(format!("Node Output -> data: {:?}", data).as_str());
    debug("[ Node Exec End ]");
}