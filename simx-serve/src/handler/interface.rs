use crate::handler::http::http::handler_http;
use crate::handler::socket::socket::handler_socket;
use simx_common::entity::flow::Transition;

pub fn handler(mut transition: Transition) -> Transition {
    match transition.node.handler.as_str() {
        "http" => { handler_http(transition.node.clone(), &mut transition.flow_data) }
        "socket" => { handler_socket(transition.node.clone(), &mut transition.flow_data) }
        _ => {}
    };
    transition
}