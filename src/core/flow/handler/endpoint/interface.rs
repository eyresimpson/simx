use std::collections::HashMap;

use crate::core::flow::entity::standardisation::Step;
use crate::core::flow::handler::endpoint::endpoint_file::interface::handle_endpoint_file;

pub fn handle_endpoint(handler_str: String, mut flow_data: HashMap<String, String>, args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    for step in steps {
        handle_endpoint_file(flow_data.clone(), args.clone(), step);
        match step.func.as_str() {
            "writer" => {
                flow_data = handle_endpoint_file_plain(flow_data.clone(), step.attr);
            }

            _ => {
                warn("Engine cannot find handle function, Skip...")
            }
        }
    }
    return flow_data;
}