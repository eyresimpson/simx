use bincode::config;
use libloading::{Library, Symbol};
use simx_common::entity::ext::{Extension, Transition};
use simx_common::entity::flow::{FlowData, Node};
use std::path::Path;

// 根据路径 + 方法名调用，参数和返回值都是String类型
pub fn call_dylib_extension_method(extension: Extension, node: Node, flow_data: &mut FlowData) -> FlowData {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();

    // 此对象需要转换为Json，然后传递给插件，插件返回的也是一个同样的对象
    // 此对象仅在同一个大版本间兼容
    let transition = Transition {
        // 引擎的大版本，除非大版本发生更新，否则不会改变
        version: 1,
        node,
        flow_data: flow_data.clone(),
    };

    // dylib路径
    let dylib_path = Path::new(&function_file).join(extension.entry_lib + ".dylib");
    let lib = unsafe { Library::new(dylib_path) }.expect("Could not load dylib");

    // 将对象序列化为二进制
    let encoded: Vec<u8> = bincode::encode_to_vec(&transition, config::standard()).unwrap();

    unsafe {
        // 获取函数符号
        // 固定调用名为interface的函数，该函数必须有一个参数，接受Vec<u8>（此处因为不符合ffi，所以只能rust程序间用）
        // 暂时放弃兼容FFI，容易出现内存泄漏的问题
        let func: Symbol<unsafe extern "C" fn(Vec<u8>) -> Vec<u8>> = lib.get(extension.entry_func.as_bytes()).expect("Could not find function");

        // 调用函数
        let result = func(encoded);
        // 将返回值处理为传输对象
        let result: Transition = bincode::decode_from_slice(&result, config::standard()).expect("TODO: panic message").0;
        // 将流数据返回回去
        result.flow_data
    }
}

pub fn call_dylib_extension_init(extension: Extension) -> Result<(), String> {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();

    // dylib路径
    let dylib_path = Path::new(&function_file).join(extension.entry_lib + ".dylib");
    let lib = unsafe { Library::new(dylib_path) }.expect("Could not load dylib");

    unsafe {
        let init: Symbol<unsafe extern "C" fn()> = lib.get("init".as_bytes()).expect("Could not find init function");
        // 调用函数
        init();
    }
    Ok(())
}
