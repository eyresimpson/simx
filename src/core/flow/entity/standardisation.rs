use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Flow {
    // 流名称
    pub(crate) flow_name: String,
    // 修改日期
    pub(crate) update_date: String,
    // 创建日期
    pub(crate) create_date: String,
    // 环境要求
    pub(crate) env_req: Vec<Env>,
    // 流程步骤集
    pub(crate) steps: Vec<Step>,
}

#[derive(Serialize, Deserialize)]
pub struct Step {
    node: Node,
    conf: NodeConf,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    current: CurrentNode,
    exec: NodeExec,
    origin: NodeOrigin,
    endpoint: NodeEndPoint,
    runtime: Runtime,
}

#[derive(Serialize, Deserialize)]
pub enum CurrentNode {
    Exec,
    Origin,
    Endpoint,
}

#[derive(Serialize, Deserialize)]
pub struct Runtime {
    input: String,
    output: String,
    variable: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeExec {}

#[derive(Serialize, Deserialize)]
pub struct NodeOrigin {}

#[derive(Serialize, Deserialize)]
pub struct NodeEndPoint {}

#[derive(Serialize, Deserialize)]
pub struct NodeConf {}

#[derive(Serialize, Deserialize)]
pub struct Env {
    env_name: String,
    env_type: String,
}