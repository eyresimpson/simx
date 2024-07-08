use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flow {
    // 流名称
    pub(crate) flow_name: String,
    // 修改日期
    pub(crate) update_date: String,
    // 创建日期
    pub(crate) create_date: String,
    // 开发者
    pub(crate) developer: String,
    // 版本
    pub(crate) version: String,
    // 环境要求
    pub(crate) env_req: Vec<Env>,
    // 节点列表
    pub(crate) nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    // 节点处理器路径，引擎会根据这个路径找到对应的handler（exec、endpoint、origin）
    // simx.exec.file.json
    // rems.cn.tineaine.exec.file.json
    pub(crate) handler: String,
    // 当前节点所附带的数据，node中的每个opt中都可以访问
    pub(crate) attr: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NodeType {
    ORIGIN,
    ENDPOINT,
    EXEC,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Env {
    pub(crate) env_key: String,
    pub(crate) env_val: String,
}

// // 标准参数
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Data {
//     pub(crate) data: HashMap<String, String>,
// }

// 流程数据
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowData {
    // 系统参数域，不要手动在代码里对其修改，属于系统自带的变量
    pub basics: HashMap<String, String>,
    // 用户参数域，可以理解为声明的变量
    pub params: HashMap<String, String>,
    // 数据统一为二进制，使用时需要根据具体情况判断
    pub data: HashMap<String, Vec<u8>>
}
