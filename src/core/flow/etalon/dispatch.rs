use std::collections::HashMap;

use crate::core::common::log::shell::info;
use crate::core::flow::entity::standardisation::{NodeType, Step};
use crate::core::flow::handler::endpoint::interface::handle_endpoint;
use crate::core::flow::handler::exec::interface::handle_exec;
use crate::core::flow::handler::origin::interface::handle_origin;

// 调度器
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub fn dispatch(handler_str: String, handler_type: NodeType, data: HashMap<String, String>, args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    info("[ Node Exec Start ]");
    // 打印提示信息
    info(format!("Node Input -> handler: {}, data: {:?}, args: {:?}", handler_str, data, args).as_str());

    // 全局流程数据对象
    let mut data: HashMap<String, String> = data;

    // 寻找对应的handler
    match handler_type {
        NodeType::ORIGIN => { data = handle_origin(handler_str, data, args, steps) }
        NodeType::ENDPOINT => { data = handle_endpoint(handler_str, data, args, steps) }
        NodeType::EXEC => { data = handle_exec(handler_str, data, args, steps) }
    }

    info(format!("Node Output -> data: {:?}", data).as_str());
    info("[ Node Exec End ]");
    return data;
}