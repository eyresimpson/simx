use crate::conf::simx::{get_config, get_net_conf};
use crate::core::engine::flow::load_and_exec_default_flow;
use crate::core::engine::script::load_and_exec_default_script;
use crate::tools::log::shell::{info, success};

pub fn init(){
    let conf = get_config();

    // 初始化脚本
    if conf.get("engine").unwrap().get("run-init-script").unwrap().as_bool().unwrap() {
        info("Default script running...");
        load_and_exec_default_script();
        success("Run init script done.");
    } else {
        info("Skip init script running.");
    }

    // 初始流
    if conf.get("engine").unwrap().get("run-init-flow").unwrap().as_bool().unwrap() {
        info("Default flow running...");
        load_and_exec_default_flow();
        success("Run init flow done.");
    } else {
        info("Skip init flow running.");
    }

}