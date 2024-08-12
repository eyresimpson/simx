use crate::conf::runtime::get_runtime_conf;
use crate::core::common::log::interface::{info, warn};
use crate::core::engine::init::scan_load_local;
use crate::core::extension::interface::call;
use crate::core::flow::handler::basic::interface::handle_basic;
use crate::core::flow::handler::db::interface::handle_db;
use crate::core::flow::handler::files::interface::handle_file;
use crate::core::flow::handler::net::interface::handle_net;
use crate::core::flow::handler::os::interface::handle_os;
use crate::entity::flow::{FlowData, Node};

pub async fn handler(node: Node, flow_data: &mut FlowData) {
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
                handle_net(node, flow_data).await;
            }
            "os" => {
                handle_os(node, flow_data);
            }
            "basic" => {
                handle_basic(node, flow_data);
            }
            _ => {
                warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[1]).as_str());
            }
        }
    } else {
        // 第一次检查, 如果插件未加载，则加载插件，这样可以实现引擎启动后再添加的插件能被正确调用
        // 后续这种方法会被淘汰，改用文件监视的方式实现
        if get_runtime_conf(format!("ext_{}", handler_path[0]).as_str()).is_none() {
            // TODO：重新刷新一遍插件，然后重试，这样可以实现所谓的插件热拔插
            info("Engine cannot find ext, Try to refresh ext list...");
            // 重新加载插件数据
            let ret = scan_load_local("ext");
            if ret.is_err() {
                warn("Engine cannot find ext, Refresh ext list failed, Skip...");
            }
        }
        // 这一步是检查是否加载了需要的插件
        if get_runtime_conf(format!("ext_{}", handler_path[0]).as_str()).is_none() {
            // 提示找不到插件
            warn(format!("Engine cannot find ext by {}, Check your ext. Flow engine skip this node...", handler_path[0]).as_str());
        }else{
            // 调用方法
            call(handler_path[0].to_string(), handler_path[1].to_string());
        }

        // 如果插件中也找不到，就直接警告跳过
        // warn("Engine cannot find handler string, Skip...");
    }
}
