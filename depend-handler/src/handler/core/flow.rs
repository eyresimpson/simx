use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{debug, fail, info, warn};
use engine_common::thread::flow::{exec_flow, exec_steps};

pub fn handle_core_flow(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 子流程
        "sub_flow" => sub_flow(node, flow_data),
        // 发起流程
        "post_flow" => {
            exec_flow(node.attr["flow_path"].clone());
        }
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

pub fn sub_flow(node: Node, flow_data: &mut FlowData) {
    debug(format!("sub flow -> {} has been called", node.attr["node_name"]).as_str());
    // 取出attr中的nodes
    if node.attr.contains_key("steps") {
        let nodes_str = node.attr.get("steps").unwrap();
        // 拆分nodes
        let nodes: Vec<Node> = match serde_json::from_str::<Vec<Node>>(nodes_str) {
            Ok(nodes) => nodes,
            Err(e) => {
                fail(format!("Failed to parse nodes_str as Vec<Node>: {}", e).as_str());
                return;
            }
        };
        let sub_flow_data = flow_data.clone();
        exec_steps(nodes, sub_flow_data);
    } else {
        info("Subflow nodes is none, Skip...");
    }
}