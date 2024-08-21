use crate::entity::flow::{FlowData, Node};
use crate::entity::simx::SimxThreadSenderStringData;
use crate::runtime::thread::get_sender_info;

pub fn exec_flow(path: String) {
    let data = SimxThreadSenderStringData {
        function: "exec_flow".to_string(),
        data: path,
    };
    let sender = get_sender_info("engine_sender");
    sender.unwrap().send(data).unwrap();
}

#[allow(unused_variables)]
pub fn exec_steps(nodes: Vec<Node>, flow_data: FlowData) {}