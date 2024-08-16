use crate::entity::flow::Transition;
use bincode::config;

mod handler;
mod tools;
pub mod entity;

// 主入口方法
#[no_mangle]
// 固定的入口函数，所有的调用都从此处开始
pub extern "C" fn interface(bytes: Vec<u8>) -> Vec<u8> {
    let transition: Transition = bincode::decode_from_slice(&bytes, config::standard()).expect("Cannot load info from bytes").0;
    println!("------> {}", transition.node.handler);
    let encoded: Vec<u8> = bincode::encode_to_vec(&transition, config::standard()).unwrap();
    encoded
}

// 初始化方法
// 引擎在初始化后，会尝试调用所有的init方法
#[no_mangle]
pub extern "C" fn init(bytes: Vec<u8>) -> Vec<u8> {
    let transition: Transition = bincode::decode_from_slice(&bytes, config::standard()).expect("Cannot load info from bytes").0;
    println!("------> {}", transition.node.handler);
    let encoded: Vec<u8> = bincode::encode_to_vec(&transition, config::standard()).unwrap();
    encoded
}