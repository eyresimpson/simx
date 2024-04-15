use std::collections::HashMap;

use crate::core::common::log::shell::info;
use crate::core::flow::entity::standardisation::Step;

pub fn handle_exec(handler_str: String, data: HashMap<String, String>, args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    info(format!("handle: {:?}, {:?}", handler_str, args).as_str());
    return data;
}