use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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
    // 流程步骤集
    pub(crate) steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Step {
    // 节点类型
    mold: StepType,
    // 节点处理器路径，引擎会根据这个路径找到对应的handler（exec、endpoint、origin）
    // simx.exec.file.create
    // remote.exec.cn.tineaine.tools.say
    handler: String,
}

enum StepType {
    ORIGIN,
    ENDPOINT,
    EXEC,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Env {
    env_name: String,
    env_type: String,
}