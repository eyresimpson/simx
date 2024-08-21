use crate::entity::simx::SimxThreadSenderStringData;
use crate::runtime::thread::get_sender_info;

#[allow(unused_variables)]
pub fn exec_script(path: String) {
    let data = SimxThreadSenderStringData {
        function: "exec_script".to_string(),
        data: path,
    };
    let sender = get_sender_info("engine_sender");
    sender.unwrap().send(data).unwrap();
}