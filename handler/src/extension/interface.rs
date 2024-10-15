use crate::extension::common::common::common_call_method;
use crate::extension::dll::interface::call_dll_extension_init;
use crate::extension::dylib::interface::call_dylib_extension_init;
use crate::extension::jar::interface::call_jar_extension_init;
use crate::extension::so::interface::call_so_extension_init;
use consts::OS;
use engine_common::entity::ext::Extension;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::warn;
use env::consts;
use std::env;

// 分离方法调用，注意只能调用Rust编写的lib，如果需要调用其他应用程序的lib，使用invoke
// simx项目中所有的内部库（比如http扩展）都是通过此方法调用
pub fn call(extension: Extension, node: Node, flow_data: &mut FlowData) -> FlowData {
    common_call_method(
        extension.entry_lib.as_str(),
        OS.to_string().to_lowercase().as_str(),
        extension.entry_func.as_str(),
        node,
        flow_data,
    )
}

pub fn call_init(extension: Extension) -> Result<(), String> {
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


// 调用第三方的库
// 这个方法兼用ffi，因此可以调用C、C++、Rust、Python等语言编写的库
pub fn invoke() -> Result<(), String> {
    Ok(())
}