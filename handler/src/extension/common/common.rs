use engine_common::entity::flow::flow::FlowData;
use engine_common::entity::flow::node::Node;
use libloader::libloading::{Library as WinLibrary, Symbol as WinSymbol};
use libloading::{Library, Symbol};
use std::path::Path;

pub fn common_call_method(lib_path: &str, os: &str, function_name: &str, node: Node, flow_data: &mut FlowData) -> FlowData {
    // 取方法所在插件文件名（相对于插件根目录）
    let dylib_path = Path::new(lib_path);

    match os {
        "win" => {
            // lib路径
            let lib = unsafe { WinLibrary::new(dylib_path) }.expect("Could not load dylib");
            unsafe {
                let func: WinSymbol<unsafe extern "C" fn(Node, FlowData) -> FlowData> = lib.get(function_name.as_ref()).expect("Could not find function");
                func(node, flow_data.clone())
            }
        }
        "linux" => {
            // lib路径
            let lib = unsafe { Library::new(dylib_path) }.expect("Could not load dylib");
            unsafe {
                let func: Symbol<unsafe extern "C" fn(Node, FlowData) -> FlowData> = lib.get(function_name.as_ref()).expect("Could not find function");
                func(node, flow_data.clone())
            }
        }
        "macos" => {
            // lib路径
            let lib = unsafe { Library::new(dylib_path) }.expect("Could not load dylib");
            unsafe {
                let func: Symbol<unsafe extern "C" fn(Node, FlowData) -> FlowData> = lib.get(function_name.as_ref()).expect("Could not find function");
                func(node, flow_data.clone())
            }
        }
        _ => panic!("Not support this platform"),
    }
}