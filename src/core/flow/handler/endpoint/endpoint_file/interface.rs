use std::collections::HashMap;

use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::Step;
use crate::core::flow::handler::endpoint::endpoint_file::plain::handle_endpoint_file_plain;

pub fn handle_endpoint_file(mut flow_data: HashMap<String, String>, args: HashMap<String, String>, step: Step) -> HashMap<String, String> {
    match step.func.as_str() {
        "writer" => {
            handle_endpoint_file_plain(flow_data.clone(), step.attr)
        }

        _ => {
            warn("Engine cannot find handle function, Skip...");
            flow_data
        }
    }
}