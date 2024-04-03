use std::collections::HashMap;
use crate::core::flow::entity::standardisation::NodeType;
use crate::core::flow::handler::endpoint::interface::handle_endpoint;
use crate::core::flow::handler::exec::interface::handle_exec;
use crate::core::flow::handler::origin::interface::handle_origin;
use crate::tools::log::shell::info;

// 调度器
// 需要传入标准的handler路径字符串和参数列表，并返回统一传回对象
pub fn dispatch(handler_str: String, handler_type: NodeType, data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {

    // 打印提示信息
    info(format!("handler_str: {}, input: {:?}, args: {:?}", handler_str, data, args).as_str());

    // 全局流程数据对象
    let mut data: HashMap<String, String> = data;

    // 寻找对应的handler
    match handler_type {
        NodeType::ORIGIN => { data = handle_origin(handler_str, data, args) }
        NodeType::ENDPOINT => { data = handle_endpoint(handler_str, data, args) }
        NodeType::EXEC => { data = handle_exec(handler_str, data, args) }
    }

    data.insert("test".parse().unwrap(), "aaa".parse().unwrap());
    return data;
}