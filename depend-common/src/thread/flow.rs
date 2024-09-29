use crate::entity::flow::{FlowData, Node, SubFlowTransferData};
use crate::entity::simx::SimxThreadSenderStringData;
use crate::runtime::thread::get_engine_sender;

pub fn exec_flow(path: String) {
    println!("=====> ");
    let data = SimxThreadSenderStringData {
        function: "exec_flow".to_string(),
        data: path,
    };
    let sender = get_engine_sender("engine_sender");
    sender.unwrap().send(data).unwrap();
}

#[allow(unused_variables)]
pub fn exec_steps(nodes: Vec<Node>, flow_data: FlowData) {
    let transfer = SubFlowTransferData {
        nodes,
        flow_data,
    };

    let data = SimxThreadSenderStringData {
        function: "exec_steps".to_string(),
        data: serde_json::to_string(&transfer).unwrap(),
    };
    let sender = get_engine_sender("engine_sender");
    sender.unwrap().send(data).unwrap();
}
