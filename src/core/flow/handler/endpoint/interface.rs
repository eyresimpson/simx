use std::collections::HashMap;

use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::Step;
use crate::core::flow::handler::endpoint::endpoint_file::interface::handle_endpoint_file;

pub fn handle_endpoint(handler_str: String, mut flow_data: HashMap<String, String>, args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    let handler_collect: Vec<&str> = handler_str.split(".").collect();
    for step in steps {
        match handler_collect[2] {
            "file" => {
                flow_data = handle_endpoint_file(flow_data.clone(), args.clone(), step);
            }

            _ => {
                warn("Engine cannot find handle function, Skip...")
            }
        }
    }
    return flow_data;
}