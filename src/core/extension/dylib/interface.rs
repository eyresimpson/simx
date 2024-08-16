use crate::entity::ext::Transition;
use crate::entity::flow::{FlowData, Node};
use bincode::config;
use libloading::{Library, Symbol};
use std::path::Path;

// 根据路径 + 方法名调用，参数和返回值都是String类型
pub fn call_dylib_extension_method(ext_path: String, function: &Vec<String>, node: Node, flow_data: &mut FlowData) -> String {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = function.get(1).unwrap().to_string();

    // 此对象需要转换为Json，然后传递给插件，插件返回的也是一个同样的对象
    // 此对象仅在同一个大版本间兼容
    let transition = Transition {
        // 引擎的大版本，除非大版本发生更新，否则不会改变
        version: 1,
        node,
        flow_data: flow_data.clone(),
    };

    // dylib路径
    let dylib_path = Path::new(&ext_path).join(function_file);
    let lib = unsafe { Library::new(dylib_path) }.expect("Could not load dylib");

    // 将对象序列化为二进制
    let encoded: Vec<u8> = bincode::encode_to_vec(&transition, config::standard()).unwrap();

    unsafe {
        // 获取函数符号
        // 固定调用名为interface的函数，该函数必须有一个参数，接受Vec<u8>（此处因为不符合ffi，所以只能rust程序间用）
        let func: Symbol<unsafe extern "C" fn(Vec<u8>) -> Vec<u8>> = lib.get("interface".as_bytes()).expect("Could not find function");

        // 调用函数
        let raw_result = func(encoded);

        let result = raw_result;
        let result: Transition = bincode::decode_from_slice(&result, config::standard()).expect("TODO: panic message").0;

        println!("Converted result: {:?}", result.node.handler);
        "".to_string()
    }
}
