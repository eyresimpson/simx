use std::collections::HashMap;

use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::Step;
use crate::core::flow::handler::origin::origin_file::plain::handle_origin_file_plain;

pub fn handle_origin(handler_str: String, mut flow_data: HashMap<String, String>, node_args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    for step in steps {
        match step.func.as_str() {
            "file_reader" => {
                flow_data = handle_origin_file_plain(flow_data.clone(), step.attr);
            }

            _ => {
                warn("Engine cannot find handle function, Skip...")
            }
        }
    }
    return flow_data;
}