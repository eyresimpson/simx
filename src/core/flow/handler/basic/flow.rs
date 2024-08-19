use crate::core::flow::exec::flow::exec_steps;
use crate::entity::flow::{FlowData, Node};
use crate::tools::log::interface::{debug, fail, info, warn};

pub async fn handle_basic_flow(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();

    match handler_path[3] {
        // 条件语句

        // 循环块

        // 子流程
        "subflow" => subflow(node, flow_data).await,

        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

pub async fn subflow(node: Node, flow_data: &mut FlowData) {
    debug(format!("sub flow -> {} has been called", node.attr["node_name"]).as_str());
    debug(format!("sub flow -> {:?} has been called", node.attr.get("steps")).as_str());
    // 取出attr中的nodes
    if node.attr.contains_key("steps") {
        let nodes_str = node.attr.get("steps").unwrap();
        println!("sub flow -> {}", nodes_str);
        // 拆分nodes
        let nodes: Vec<Node> = match serde_json::from_str::<Vec<Node>>(nodes_str) {
            Ok(nodes) => nodes,
            Err(e) => {
                fail(format!("Failed to parse nodes_str as Vec<Node>: {}", e).as_str());
                return;
            }
        };
        let sub_flow_data = flow_data.clone();
        exec_steps(nodes, sub_flow_data).await;
    } else {
        info("Subflow nodes is none, Skip...");
    }
}