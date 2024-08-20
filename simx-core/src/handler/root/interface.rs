use simx_common::entity::flow::{FlowData, Node};

pub fn handle_root(node: Node, flow_data: FlowData) -> FlowData {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[1] {
        "destroy" => {
            // 销毁方法
        }
        "info" => {
            // 库信息
        }
        _ => {
            // 提示找不到方法
        }
    }
    // 初始化方法
    // 销毁方法
    // 库信息
    return flow_data;
}