use engine_common::entity::error::NodeError;
use engine_common::entity::error::NodeError::{HandleNotFound, HandleRuntimeError};
use engine_common::entity::flow::{FlowData, Node};
use engine_common::thread::flow::{exec_flow, exec_steps};

pub fn handle_core_flow(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 子流程
        "sub_flow" => sub_flow(node, flow_data),
        // 发起流程
        "post_flow" => {
            match node.attr.get("flow_path") {
                Some(path) => {
                    let path = path.as_str().expect("flow_path is not string");
                    match exec_flow(path.to_string()) {
                        Ok(_) => Ok(()),
                        Err(e) => return Err(HandleRuntimeError(e))
                    }
                }
                None => Err(NodeError::ParamFormatError("flow_path is none".to_string()))
            }
        }
        _ => {
            Err(HandleNotFound(node.handler))
        }
    }
}

pub fn sub_flow(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.attr.get("steps") {
        Some(steps) => {
            let nodes_str = steps.as_str().expect("steps is not string");
            // 拆分nodes
            let nodes: Vec<Node> = match serde_json::from_str::<Vec<Node>>(nodes_str) {
                Ok(nodes) => nodes,
                Err(e) => {
                    return Err(NodeError::ParamFormatError(e.to_string()));
                }
            };
            let sub_flow_data = flow_data.clone();
            exec_steps(nodes, sub_flow_data);
            Ok(())
        }
        None => {
            Err(NodeError::ParamNotFound("steps".to_string()))
        },
    }
}