use crate::core::flow::handler::os::shell::handle_os_shell_println;
use crate::tools::log::interface::warn;
use simx_common::entity::flow::{FlowData, Node};

pub fn handle_os(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        // 执行shell命令
        "shell" => handle_os_shell_println(node, flow_data),
        // 获取所有进程
        "process" => {}
        // 查询指定进程是否存活
        "alive" => {}
        // 启动指定进程
        "start" => {}
        // 停止指定进程
        "kill" => {}
        // 获取操作系统信息
        "info" => {}
        // 锁定系统
        "lock" => {}
        // 进入睡眠模式
        "sleep" => {}
        // 注销当前用户
        "logout" => {}
        // 重启系统
        "reboot" => {}
        // 关闭系统
        "shutdown" => {}
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[2]).as_str());
        }
    }
}
