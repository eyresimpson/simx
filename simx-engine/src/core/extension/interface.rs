use crate::core::extension::dll::interface::{call_dll_extension_init, call_dll_extension_method};
use crate::core::extension::dylib::interface::{call_dylib_extension_init, call_dylib_extension_method};
use crate::core::extension::jar::interface::{call_jar_extension_init, call_jar_extension_method};
use crate::core::extension::so::interface::{call_so_extension_init, call_so_extension_method};
use crate::tools::log::interface::warn;
use consts::OS;
use env::consts;
use simx_common::entity::ext::Extension;
use simx_common::entity::flow::{FlowData, Node};
use std::env;

pub fn call(extension: Extension, node: Node, flow_data: &mut FlowData) -> FlowData {
    let ext = extension.clone();
    let path = ext.path.expect("Extension path is none");
    let file_name = ext.entry_lib;
    if file_name.ends_with(".jar") {
        return call_jar_extension_method(path.to_string(), node, flow_data);
    } else if file_name.ends_with(".py") {
        warn("Not support py file yet");
    } else {
        // 可能调用的与平台有关的库，比如dll、so、或dylib
        // 判断当前操作系统是windows、linux还是macos
        match OS.to_string().to_lowercase().as_str() {
            "windows" => {
                return call_dll_extension_method(extension, node, flow_data);
            }
            "linux" => {
                return call_so_extension_method(extension, node, flow_data);
            }
            "macos" => {
                return call_dylib_extension_method(extension, node, flow_data)
            }
            _ => {}
        }
    }
    warn(format!("Function not found in extension {}", extension.name).as_str());
    flow_data.clone()
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