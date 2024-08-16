use crate::entity::flow::{FlowData, Node};

pub fn handle_sys(node: Node, flow_data: FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[1] {
        // 信息（如获取内存、ip等操作）
        "info" => {
            // 初始化方法
        }
        // 管理（如修改开机自启动、修改系统时区等操作）
        "manage" => {
            // 运行方法
        }
        // 进程（如启动、停止进程）
        "process" => {}
        // 控制（如开关机）
        "control" => {}
        _ => {
            // 提示找不到方法
        }
    }
}