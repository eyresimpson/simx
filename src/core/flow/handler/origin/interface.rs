use std::collections::HashMap;

use crate::core::common::log::shell::info;
use crate::core::flow::entity::standardisation::Step;
use crate::core::flow::handler::origin::origin_file::plain::handle_origin_file_plain;

pub fn handle_origin(handler_str: String, mut flow_data: HashMap<String, String>, node_args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    info(format!("handle: {:?}, {:?}", handler_str, node_args).as_str());
    for step in steps {
        match step.func {
            // => {}
            String { .. } => {
                flow_data = handle_origin_file_plain(flow_data.clone(), step.attr);
            }
        }
    }
    return flow_data;
}