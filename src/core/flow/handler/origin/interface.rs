use std::collections::HashMap;

use crate::core::common::log::shell::info;
use crate::core::flow::handler::origin::origin_file::plain::handle_origin_file_plain;

pub fn handle_origin(handler_str: String, data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    info(format!("handle: {:?}, {:?}", handler_str, args).as_str());
    // 尝试根据名称解析出要使用的方法（动态）
    match handler_str {
         // => {}
        String { .. } => {
            handle_origin_file_plain(handler_str, data.clone(), args);
        }
    }
    return data;
}