use crate::entity::exception::node::NodeError;
use crate::entity::flow::flow::FlowData;
use crate::entity::flow::node::Node;
use crate::runtime::extension::get_extension_library;
use libloader::libloading::Symbol as WinSymbol;
use libloading::Symbol;
use std::env::consts::OS;
use std::path::{Path, PathBuf};

pub fn common_call_method(
    path: &str,
    os: &str,
    function_name: &str,
    node: Node,
    flow_data: &mut FlowData,
) -> Result<(), NodeError> {
    match os {
        "windows" => {
            let lib = get_extension_library(path)?.win.unwrap();
            unsafe {
                let func: WinSymbol<unsafe extern "C" fn(Node, &mut FlowData) -> Result<(), NodeError>> = lib.get(function_name.as_ref()).expect("Could not find function");
                func(node, flow_data)
            }
        }
        "linux" => {
            let lib = get_extension_library(path)?.linux.unwrap();
            unsafe {
                let func: Symbol<unsafe extern "C" fn(Node, &mut FlowData) -> Result<(), NodeError>> = lib.get(function_name.as_ref()).expect("Could not find function");
                func(node, flow_data)
            }
        }
        "macos" => {
            let lib = get_extension_library(path)?.mac.unwrap();
            unsafe {
                let func: Symbol<unsafe extern "C" fn(Node, &mut FlowData) -> Result<(), NodeError>> = lib.get(function_name.as_ref()).expect("Could not find function");
                func(node, flow_data)
            }
        }
        _ => panic!("Not support this platform"),
    }
}

// 组装插件的真实路径
pub fn get_extension_path(path: String, entry_lib: String) -> PathBuf {
    let os = OS.to_string().to_lowercase();
    match os.as_str() {
        "windows" => {
            Path::new(&path).join(entry_lib + ".dll")
        }
        "linux" => {
            Path::new(&path).join(entry_lib + ".so")
        }
        "macos" => {
            Path::new(&path).join(entry_lib + ".dylib")
        }
        _ => {
            Path::new(&path).join(entry_lib + ".so")
        }
    }
}