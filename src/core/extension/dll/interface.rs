use std::any::Any;
use std::ffi::c_void;
use std::path::Path;

use libloader::libloading::{Library, Symbol};
use crate::core::common::log::interface::debug;

// 根据路径 + 方法名调用
pub fn call_dll_extension_method(ext_path: String, function: &Vec<String>) {

    // 取方法名
    let function_name = function.get(0).unwrap().to_string();
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = function.get(1).unwrap().to_string();
    // 取方法描述（x）
    // let function_description = function.get(2).unwrap().to_string();
    // TODO：取方法返回值
    // let function_return = function.get(3).unwrap().to_string();
    // 取方法参数
    let function_params = function[4..].to_vec();

    // dll路径
    let dll_path = Path::new(&ext_path).join(function_file);
    let lib = unsafe { Library::new(dll_path) }.expect("Could not load DLL");

    debug(format!("Calling function {:?} in file {:?}", function.get(0).unwrap().to_string(), function.get(1).unwrap().to_string()).as_str());

    // 准备参数
    let params: Vec<Box<dyn Any>> = function_params.iter().map(|p| {
        match p.split(' ').next().unwrap() {
            // 整数
            "int" => Box::new(42) as Box<dyn Any>,
            // 字符串
            "string" => Box::new("") as Box<dyn Any>,
            // 浮点
            "double" => Box::new(0.01) as Box<dyn Any>,
            // 高精度浮点
            "float" => Box::new(0.01) as Box<dyn Any>,
            // 布尔
            "boolean" => Box::new(false) as Box<dyn Any>,
            // 对象，Json对象
            "object" => Box::new("{}") as Box<dyn Any>,
            // 数组,Json数组
            "array" => Box::new("[]") as Box<dyn Any>,
            // 处理其他类型...
            _ => panic!("Unsupported parameter type"),
        }
    }).collect();

    unsafe {
        // 获取函数符号
        let func: Symbol<unsafe extern "C" fn(*const c_void) -> *const c_void> = lib.get(function_name.as_bytes()).expect("Could not find function");

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