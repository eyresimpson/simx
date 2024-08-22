use crate::extension::interface::call;
use crate::handler::basic::interface::handle_basic;
use crate::handler::files::interface::handle_file;
use crate::handler::net::interface::handle_net;
use crate::handler::os::interface::handle_os;
use crate::handler::script::interface::handle_script;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{info, warn};
use engine_common::runtime::extension::get_extension_info;
use engine_common::thread::engine::reload_local;

pub fn root_handler(node: Node, flow_data: &mut FlowData) -> Result<(), String> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    // 判断是否为内置 handler，内置的handler必然以simx开头
    if handler_path[0] == "simx" {
        // 此处采用match方式直接匹配，可以大幅度增加速度
        // 此处的功能并不多，引擎主体本身希望增加速度，所以采用match方式
        match handler_path[1] {
            // 基础文件
            "files" => {
                handle_file(node, flow_data);
            }
            // 基础网络
            "net" => {
                handle_net(node, flow_data);
            }
            // 系统功能
            "os" => {
                handle_os(node, flow_data);
            }
            // 基础操作
            "basic" => {
                handle_basic(node, flow_data);
            }
            // 调用脚本（脚本引擎）
            "script" => {
                handle_script(node);
            }
            _ => {
                warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[1]).as_str());
            }
        }
    } else {
        // 第一次检查, 如果插件未加载，则加载插件，这样可以实现引擎启动后再添加的插件能被正确调用
        // TODO: 后续这种方法会被淘汰，改用文件监视的方式实现
        if get_extension_info(handler_path[0]).is_none() {
            // 重新刷新一遍插件，然后重试，这样可以实现所谓的插件热拔插
            info("Engine cannot find ext, Try to refresh ext list...");
            // 重新加载插件数据
            let ret = reload_local("ext");
            if ret.is_err() {
                warn("Engine cannot find ext, Refresh ext list failed, Skip...");
            }
        }
        let extension = get_extension_info(handler_path[0]);
        if extension.is_none() {
            // 提示找不到插件
            Err(format!("Engine cannot find ext by {}, Check your ext. Flow engine skip this node...", handler_path[0]))
        } else {
            // 调用方法
            call(extension.unwrap(), node, flow_data);
            Ok(())
        }?
    }
    Ok(())
}
