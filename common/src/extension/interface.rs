use crate::entity::extension::Extension;
use crate::entity::flow::flow::FlowData;
use crate::entity::flow::node::Node;
use crate::extension::common::common::common_call_method;
use crate::extension::dll::interface::call_dll_extension_init;
use crate::extension::dylib::interface::call_dylib_extension_init;
use crate::extension::jar::interface::call_jar_extension_init;
use crate::extension::so::interface::call_so_extension_init;
use crate::logger::interface::{info, warn};
use std::env::consts::OS;

// 加载扩展
pub fn load_extension(extension: Extension) {}

// 卸载扩展
pub fn unload_extension(extension: Extension) {}

// 检查扩展
pub fn check_extension(extension: Extension) {}

// 调用rust编写的扩展（直接是结构体）
pub fn invoke_extension_func_common(extension: Extension, node: Node, flow_data: &mut FlowData) -> FlowData {
    common_call_method(
        extension.entry_lib.as_str(),
        OS.to_string().to_lowercase().as_str(),
        extension.handle_func.as_str(),
        node,
        flow_data,
    )
}

// 调用非rust编写的扩展（通过二进制或Json字符串）
pub fn invoke_extension_func_natural() {}

// 调用脚本接口
pub fn invoke_extension_func_script() {}

// 调用restful接口
pub fn invoke_extension_func_restful() {}

// 调用socket接口
pub fn invoke_extension_func_socket() {}

// 调用扩展的init
pub fn call_extension_init(extension: Extension) -> Result<(), String> {
    info(format!("Try to call extension {} init", extension.name).as_str());
    let ext = extension.clone();
    ext.path.expect("Extension path is none");
    let file_name = ext.entry_lib;
    if file_name.ends_with(".jar") {
        return call_jar_extension_init(extension);
    } else if file_name.ends_with(".py") {
        warn("Not support py file yet");
    } else {
        // 可能调用的与平台有关的库，比如dll、so、或dylib
        // 判断当前操作系统是windows、linux还是macos
        match OS.to_string().to_lowercase().as_str() {
            "windows" => {
                return call_dll_extension_init(extension);
            }
            "linux" => {
                return call_so_extension_init(extension);
            }
            "macos" => {
                return call_dylib_extension_init(extension)
            }
            _ => {}
        }
    }
    warn(format!("Function not found in extension {}", extension.name).as_str());
    Ok(())
}

pub fn call_extension_destroy(extension: Extension) -> Result<(), String> {
    Ok(())
}