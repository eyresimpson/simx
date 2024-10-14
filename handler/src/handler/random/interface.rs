use crate::handler::random::infomation::interface::handle_random_info;
use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use rand::Rng;

pub fn handle_random(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // 获取一个随机数
        "getNum" => { get_random_num(node, flow_data) }
        // 随机获取一个String字符串
        "getStr" => { Ok(()) }
        // 随机获取一个UUID字符串
        "getUUID" => { Ok(()) }
        // 随机生成一个日期
        "getDate" => { Ok(()) }
        // 随机用户信息生成
        "info" => handle_random_info(node, flow_data),
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}

// 生成一个随机数字
fn get_random_num(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    // 随机生成数字
    let num = rand::thread_rng().gen_range(1..=100);
    // 将结果写入到节点数据中
    flow_data.nodes.insert(node.id.unwrap(), num.to_string().into());
    Ok(())
}