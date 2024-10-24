use crate::entity::exception::node::NodeError;
use crate::entity::flow::flow::FlowData;
use crate::entity::flow::node::Node;
use crate::runtime::extension::get_extension_library;
use libloader::libloading::Symbol as WinSymbol;
use libloading::Symbol;

pub fn common_call_method(
    path: &str,
    os: &str,
    function_name: &str,
    node: Node,
    flow_data: &mut FlowData,
) -> Result<(), NodeError> {
    match os {
        "win" => {
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