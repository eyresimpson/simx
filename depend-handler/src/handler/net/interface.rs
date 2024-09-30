use crate::handler::net::http::handle_net_http;
use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};

pub async fn handle_net(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "http" => {
            handle_net_http(node, flow_data).await;
            Ok(())
        }
        "https" => { Ok(()) }
        "tcp" => { Ok(()) }
        "udp" => { Ok(()) }
        _ => {
            Err(NodeError::HandleNotFound(node.handler))
        }
    }
}