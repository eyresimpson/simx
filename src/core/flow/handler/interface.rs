use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::{Data, Node};
use crate::core::flow::handler::db::interface::handle_db;
use crate::core::flow::handler::files::interface::handle_file;
use crate::core::flow::handler::net::interface::handle_net;
use crate::core::flow::handler::os::interface::handle_os;

pub fn handler(node: Node, flow_data: &mut Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    // 判断是否为内置 handler，内置的handler必然以simx开头
    if handler_path[0] == "simx" {
        // 这个地方后续改成动态调用方法
        // TODO: 改成动态调用而不是通过match
        match handler_path[1] {
            "files" => {
                handle_file(node, flow_data);
            }
            "db" => {
                handle_db(node, flow_data);
            }
            "net" => {
                handle_net(node, flow_data);
            }
            "os" => {
                handle_os(node, flow_data);
            }
            _ => {
                warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[1]).as_str());
            }
        }
    } else {
        // 如果不是内置的handler
    }
}
