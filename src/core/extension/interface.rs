use crate::core::extension::dll::interface::call_dll_extension_method;
use crate::core::extension::dylib::interface::call_dylib_extension_method;
use crate::core::extension::jar::interface::call_jar_extension_method;
use crate::core::extension::so::interface::call_so_extension_method;
use crate::entity::ext::Extension;
use crate::entity::flow::{FlowData, Node};
use crate::tools::log::interface::warn;
use consts::OS;
use env::consts;
use std::env;

pub fn call(extension: Extension, function_name: String, node: Node, flow_data: &mut FlowData) {
    let functions = extension.function;
    for function in functions {
        if function[0].eq(function_name.as_str()) {
            if function[1].ends_with(".jar") {
                call_jar_extension_method(extension.path.unwrap().to_string(), &function);
            } else if function[1].ends_with(".py") {
                warn("Not support py file yet");
            } else {
                // 可能调用的与平台有关的库，比如dll、so、或dylib
                // 判断当前操作系统是windows、linux还是macos
                match OS.to_string().to_lowercase().as_str() {
                    "windows" => {
                        let mut fun = function;
                        fun[1] = format!("{}.dll", fun[1]);
                        call_dll_extension_method(extension.path.unwrap().to_string(), &fun, node, flow_data);
                    }
                    "linux" => {
                        let mut fun = function;
                        fun[1] = format!("{}.so", fun[1]);
                        call_so_extension_method(extension.path.unwrap().to_string(), &fun, node, flow_data);
                    }
                    "macos" => {
                        // 作为macos，需要将文件名改为dylib
                        let mut fun = function;
                        fun[1] = format!("{}.dylib", fun[1]);
                        call_dylib_extension_method(extension.path.unwrap().to_string(), &fun, node, flow_data);
                    }
                    _ => {}
                }
            }
            return;
        }
    }
    warn(format!("Function {} not found in extension {}", function_name, extension.name).as_str());
}