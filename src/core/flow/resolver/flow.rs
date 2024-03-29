use std::path::Path;

use crate::core::flow::entity::standardisation::Flow;
use crate::tools::log::shell::info;

pub fn resolver_flow(path: &Path) -> Flow{
    info(path.to_str().unwrap());
    // 统一流程对象
    let flow = Flow{
        flow_name: "".parse().unwrap(),
        update_date: "".parse().unwrap(),
        create_date: "".parse().unwrap(),
        env_req:Vec::new(),
        steps:Vec::new(),
    };
    // let ret = Flow::serialize(&flow, ()).unwrap();
    return flow;
}