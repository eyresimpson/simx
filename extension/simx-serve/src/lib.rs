use crate::entity::flow::Transition;
use crate::handler::interface::handler;
use crate::handler::serve::interface::serve;
use bincode::config;

mod handler;
mod tools;
pub mod entity;

#[no_mangle]
// 固定的入口函数，所有的调用都从此处开始
pub extern "C" fn interface(bytes: Vec<u8>) -> Vec<u8> {
    let mut transition: Transition = bincode::decode_from_slice(&bytes, config::standard()).expect("Cannot load info from bytes").0;
    transition = handler(transition.clone());
    let encoded: Vec<u8> = bincode::encode_to_vec(&transition, config::standard()).unwrap();
    encoded
}

#[no_mangle]
// 固定的入口函数，所有的调用都从此处开始
pub extern "C" fn init() {
    println!("init complete");
    serve();
}