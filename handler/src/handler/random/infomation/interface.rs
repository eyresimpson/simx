use engine_common::entity::error::NodeError;
use engine_common::entity::flow::Node;

pub fn handle_random_info(node: Node) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 随机生成一个中文名称
        "getChineseName" => { Ok(()) }
        // 随机生成一个英文名称
        "getEnglishName" => { Ok(()) }
        // 随机生成一个手机号
        "getPhone" => { Ok(()) }
        // 随机生成一个邮箱
        "getEmail" => { Ok(()) }
        // 随机生成一个身份证号
        "getIDCard" => { Ok(()) }
        // 随机生成一个地址
        "getAddress" => { Ok(()) }
        // 随机生成一个密码
        "getPassword" => { Ok(()) }
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}