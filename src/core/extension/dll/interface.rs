// use std::path::Path;
//
// use crate::core::common::log::interface::debug;
//
// pub fn load_dll_extension(path: &Path) {
//     debug(format!("Load dll extension from {}", path.display()).as_str());
//
// }

use std::ffi::c_void;
use std::fs;
use std::path::Path;
use libloader::libloading::{Library, Symbol};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct FunctionParam {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
    description: String,
}

#[derive(Deserialize)]
struct FunctionResult {
    #[serde(rename = "type")]
    result_type: String,
    description: String,
}

#[derive(Deserialize)]
struct Function {
    name: String,
    filename: String,
    description: String,
    isAsync: bool,
    params: Vec<FunctionParam>,
    result: FunctionResult,
}

#[derive(Deserialize)]
struct Extension {
    name: String,
    version: String,
    description: String,
    license: String,
    author: String,
    keywords: Vec<String>,
    dependencies: Vec<String>,
    function: Vec<Function>,
}

// 根据路径 + 方法名调用
pub fn call_extension_method(path: String, method: String) {
    // 读取 JSON 文件
    let file_path = Path::new("D:\\Code\\simx-core\\release\\extension.json");
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let extension: Extension = serde_json::from_str(&data).expect("JSON was not well-formatted");

    // 动态加载 DLL
    // dll路径
    let dll_path = Path::new(&path);
    let lib = unsafe { Library::new(dll_path) }.expect("Could not load DLL");

    println!("Calling function {}", method);

    // 准备参数
    let params: Vec<Box<dyn std::any::Any>> = extension.function[0].params.iter().map(|p| {
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
        let func: Symbol<unsafe extern "C" fn(*const c_void) -> *const c_void> = lib.get(method.as_bytes()).expect("Could not find function");

        // 准备参数指针
        let args: Vec<*const c_void> = params.iter().map(|p| p.as_ref() as *const _ as *const c_void).collect();

        // 调用函数
        let result_ptr = func(args.as_ptr() as *const c_void);

        // 处理返回值
        match extension.function[0].result.result_type.as_str() {
            // 返回值为空
            "None" => {
                let result: i32 = *(result_ptr as *const i32);
                println!("The result of the function is: {}", result);
            }
            // 整数类型
            "Int" => {
                let result: i32 = *(result_ptr as *const i32);
                println!("The result of the function is: {}", result);
            }
            // 浮点数类型
            "Float" => {
                let result: f32 = *(result_ptr as *const f32);
                println!("The result of the function is: {}", result);
            }
            // 字符串类型
            "String" => {
                let result: f32 = *(result_ptr as *const f32);
                println!("The result of the function is: {}", result);
            }
            // Json类型（其实是字符串，但转换为Json），标准返回值
            "Json" => {
                let result: f32 = *(result_ptr as *const f32);
                println!("The result of the function is: {}", result);
            }
            // Bytes数组类型
            "Bytes" => {
                let result: f32 = *(result_ptr as *const f32);
                println!("The result of the function is: {}", result);
            }
            // 处理其他返回类型...
            _ => panic!("Unsupported result type"),
        }
    }
}