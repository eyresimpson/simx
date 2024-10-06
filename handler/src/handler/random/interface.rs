use crate::handler::random::infomation::interface::handle_random_info;
use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use rand::Rng;

pub fn handle_random(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // 获取一个随机数
        "getNum" => { get_random_num() }
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
    // flow_data.data.insert(node.name.clone());
    Ok(())
}
fn generate_random_number(number_type: &str, digit_count: usize, range: (i32, i32)) -> Result<f64, String> {
    let (min, max) = range;
    if min > max {
        return Err("最小值不能大于最大值".to_string());
    }

    match number_type {
        "int" => {
            let mut rng = rand::thread_rng();
            if digit_count == 0 {
                Ok(rng.gen_range(min..=max) as f64)
            } else {
                let min_digit = 10_i32.pow((digit_count - 1) as u32);
                let max_digit = 10_i32.pow(digit_count as u32) - 1;
                if min < min_digit || max > max_digit {
                    Err(format!("指定的范围({},{})与位数({})不匹配", min, max, digit_count))
                } else {
                    Ok(rng.gen_range(min..=max) as f64)
                }
            }
        }
        "float" => {
            let mut rng = rand::thread_rng();
            Ok(rng.gen_range(min as f64..=max as f64))
        }
        _ => Err("未知的数字类型，请指定 'int' 或 'float'".to_string()),
    }
}