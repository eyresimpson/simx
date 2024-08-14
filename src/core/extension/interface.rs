use crate::core::extension::dll::interface::call_dll_extension_method;
use crate::entity::ext::Extension;
use crate::tools::log::interface::warn;

pub fn call(extension: Extension, function_name: String) {
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