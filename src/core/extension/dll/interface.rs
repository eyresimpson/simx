use std::ffi::c_void;
use std::path::Path;

use libloader::libloading::{Library, Symbol};

use crate::entity::ext::Function;

// 根据路径 + 方法名调用
pub fn call_extension_method(path: String, function: Function) {

    // dll路径
    let dll_path = Path::new(&path);
    let lib = unsafe { Library::new(dll_path) }.expect("Could not load DLL");

    println!("Calling function {}", function.name);

    // 准备参数
    let params: Vec<Box<dyn std::any::Any>> = function.params.iter().map(|p| {
        match p.param_type.as_str() {
            // 没有参数，空参数
            "None" => Box::new(3.14) as Box<dyn std::any::Any>, // 示例参数
            "Int" => Box::new(42) as Box<dyn std::any::Any>, // 示例参数
            "Float" => Box::new(3.14) as Box<dyn std::any::Any>, // 示例参数
            // 处理其他类型...
            _ => panic!("Unsupported parameter type"),
        }
    }).collect();

    unsafe {
        // 获取函数符号
        let func: Symbol<unsafe extern "C" fn(*const c_void) -> *const c_void> = lib.get(function.name.as_bytes()).expect("Could not find function");

        // 准备参数指针
        let args: Vec<*const c_void> = params.iter().map(|p| p.as_ref() as *const _ as *const c_void).collect();

        // 调用函数
        // let result_ptr = func(args.as_ptr() as *const c_void);
        func(args.as_ptr() as *const c_void);

        // // 处理返回值
        // match function.result.result_type.as_str() {
        //     // 返回值为空
        //     "None" => {
        //         let result: i32 = *(result_ptr as *const i32);
        //         println!("The result of the function is: {}", result);
        //     }
        //     // 整数类型
        //     "Int" => {
        //         let result: i32 = *(result_ptr as *const i32);
        //         println!("The result of the function is: {}", result);
        //     }
        //     // 浮点数类型
        //     "Float" => {
        //         let result: f32 = *(result_ptr as *const f32);
        //         println!("The result of the function is: {}", result);
        //     }
        //     // 字符串类型
        //     "String" => {
        //         let result: f32 = *(result_ptr as *const f32);
        //         println!("The result of the function is: {}", result);
        //     }
        //     // Json类型（其实是字符串，但转换为Json），标准返回值
        //     "Json" => {
        //         let result: f32 = *(result_ptr as *const f32);
        //         println!("The result of the function is: {}", result);
        //     }
        //     // Bytes数组类型
        //     "Bytes" => {
        //         let result: f32 = *(result_ptr as *const f32);
        //         println!("The result of the function is: {}", result);
        //     }
        //     // 处理其他返回类型...
        //     _ => panic!("Unsupported result type"),
        // }
    }
}