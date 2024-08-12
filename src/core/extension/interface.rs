use serde_json::from_str;
use crate::conf::runtime::get_runtime_conf;
use crate::core::common::log::interface::warn;
use crate::core::extension::dll::interface::call_dll_extension_method;
use crate::entity::ext::Extension;

pub fn call(extension_name: String, function_name: String) {
    let extension: Extension = from_str(get_runtime_conf(format!("ext_{}", extension_name).as_str()).unwrap().as_str()).unwrap();
    let functions = extension.function;
    for function in functions {
        if function[0].eq(function_name.as_str()) {
            if function[1].ends_with(".dll") {
                call_dll_extension_method(extension.path.unwrap().to_string(), &function)
            } else if function[1].ends_with(".so") {
                warn("Not support so file yet");
            } else if function[1].ends_with(".jar") {
                warn("Not support jar file yet");
            } else if function[1].ends_with(".py") {
                warn("Not support py file yet");
            }
            return;
        }
    }
    warn(format!("Function {} not found in extension {}", function_name, extension.name).as_str());
}