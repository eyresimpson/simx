use crate::handler::root::interface::handle_root;
use simx_common::entity::flow::Transition;

pub fn handler(mut transition: Transition) -> Transition {
    match transition.node.handler.as_str() {
        "root" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        "sys" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        "net" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        "file" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        "doc" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        "db" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        _ => {}
    };
    transition
}