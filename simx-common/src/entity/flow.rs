use bincode::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flow {
    // 流名称
    pub flow_name: String,
    // 修改日期
    pub update_date: String,
    // 创建日期
    pub create_date: String,
    // 开发者
    pub developer: String,
    // 版本
    pub version: String,
    // 环境要求
    pub requirements: Vec<Environment>,
    // 节点列表
    pub steps: Vec<Node>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode)]
pub struct Node {
    // 节点处理器路径，引擎会根据这个路径找到对应的handler
    pub handler: String,
    // 当前节点所附带的数据，node中的每个opt中都可以访问
    pub attr: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvType {
    // 基本
    Base,
    // 运行时
    RT,
    // 扩展插件
    Ext,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Environment {
    pub name: String,
    pub env_type: EnvType,
    pub ver: String,
}

// // 标准参数
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Data {
//     pub data: HashMap<String, String>,
// }

// 流程数据
#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode)]
pub struct FlowData {
    // 系统参数域，不要手动在代码里对其修改，属于系统自带的变量
    pub basics: HashMap<String, String>,
    // 用户参数域，可以理解为声明的变量
    pub params: HashMap<String, String>,
    // 数据统一为二进制，使用时需要根据具体情况判断
    pub data: HashMap<String, Vec<u8>>,
}
