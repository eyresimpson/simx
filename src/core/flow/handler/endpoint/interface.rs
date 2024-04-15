use std::collections::HashMap;

use crate::core::common::log::shell::{info, warn};
use crate::core::flow::entity::standardisation::Step;
use crate::core::flow::handler::endpoint::endpoint_file::plain::handle_endpoint_file_plain;

pub fn handle_endpoint(handler_str: String, mut flow_data: HashMap<String, String>, args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    for step in steps {
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