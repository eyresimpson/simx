use crate::entity::flow::Transition;
use crate::handler::http::http::handler_http;

pub fn handler(mut transition: Transition) -> Transition {
    match transition.node.handler.as_str() {
        "http" => { handler_http(transition.node.clone(), &mut transition.flow_data) }
        // "sys" => { transition.flow_data = handle_root(transition.node.clone(), transition.flow_data.clone()) }
        _ => {}
    };
    transition
}