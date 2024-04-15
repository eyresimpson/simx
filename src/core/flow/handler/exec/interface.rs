use std::collections::HashMap;

use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::Step;
use crate::core::flow::handler::exec::exec_system::shell::handle_exec_system_shell;

pub fn handle_exec(handler_str: String, mut flow_data: HashMap<String, String>, node_args: HashMap<String, String>, steps: Vec<Step>) -> HashMap<String, String> {
    for step in steps {
        match step.func.as_str() {
            "shell" => {
                flow_data = handle_exec_system_shell(flow_data.clone(), step.attr);
            }

            _ => {
                warn("Engine cannot find handle function, Skip...")
            }
        }
    }
    return flow_data;
}