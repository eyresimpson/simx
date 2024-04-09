use std::collections::HashMap;

use crate::core::common::log::shell::info;

pub fn handle_endpoint(handler_str: String, data: HashMap<String, String>, args: HashMap<String, String>)-> HashMap<String, String>{
    info(format!("handle: {:?}, {:?}", handler_str, args).as_str());
    return data;
}