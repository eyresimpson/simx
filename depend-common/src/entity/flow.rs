use bincode::{Decode, Encode};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Flow {
    // 流文件版本
    pub version: String,
    // 流名称
    pub name: String,
    // 修改日期
    pub update_date: String,
    // 创建日期
    pub create_date: String,
    // 开发者
    pub developer: String,
    // 环境要求
    pub requirements: Vec<Environment>,
    // 执行蓝图
    pub blueprint: Blueprint,
    // 流运行时，此字段在调度器中赋值与管理
    pub runtime: Option<FlowRuntimeModel>,
}

// 流程状态模型
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowRuntimeModel {
    //  流当前状态
    pub status: FlowStatus,
    // 执行历史（记录节点的id）
    pub history: HashMap<String, NodeHistory>,
    // 流日志
    pub logs: Vec<NodeMessage>,
    // 当前节点
    pub current_node: Option<Node>,
    // 流运行时数据
    pub data: FlowData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FlowStatus {
    // 启动中（尝试执行）
    // 进入exec阶段，会让状态变为starting
    Starting,
    // 队列中
    // 以下情况会进入队列：
    // 1. 生成流状态后，如果用户限制了最大线程数，且占用的线程超过最大线程数
    Queue,
    // 正在运行
    Running,
    // 已完成（正常结束）
    Finished,
    // 发生错误终止
    // 1. 节点不允许错误，会引发此状态
    // 2. 调度失败（如内存分配等情况）
    Error,
    // 调度暂停
    // 用户手动暂停，或节点触发暂停，会引发此状态
    Paused,
    // 调度阻塞（超过限制）
    Waiting,
    // 状态未知（失控/被提前销毁）
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode)]
pub struct Node {
    pub name: String,
    // 节点标签列表
    pub tags: Option<Vec<NodeTag>>,
    // 节点处理器路径，引擎会根据这个路径找到对应的handler
    pub handler: String,
    // 当前节点的配置
    pub attr: HashMap<String, String>,
    // 下游节点id列表
    pub downstream: Vec<String>,
    // 补偿流id列表
    pub redress_stream: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode, PartialEq)]
pub enum NodeTag {
    // 计算节点，具有大量计算负荷
    Compute,
    // 命令节点，与操作系统进行命令交互
    Command,
    // 路由节点，会在节点执行结束后，要求调整执行路径
    Route,
    // 数据节点，与数据库、数据文件进行交互
    Data,
    // 测试节点，仅用于调试和开发
    Debug,
    // 耗时节点，比如与第三方接口进行交互
    Delay,
    // 优先节点，会优先处理此节点
    Priority,
    // 阻塞节点，会阻塞调度器，直到被取消
    Blocking,
    // 异步节点，不会等待此节点执行
    Async,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode)]
pub struct NodeMessage {
    pub message: String,
    pub data: Option<Vec<u8>>,
    pub level: i32,
    pub time: String,
    pub node_id: String,
    pub message_type: NodeMessageType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode)]
pub enum NodeMessageType {
    Info,
    Warning,
    Error,
    Debug,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode)]
pub struct NodeHistory {
    // 节点处理器路径，引擎会根据这个路径找到对应的handler
    pub id: String,
    // 当前节点所附带的数据，node中的每个opt中都可以访问
    pub attr: HashMap<String, String>,
    // 输入流
    pub input_data: FlowData,
    // 输出流
    pub output_data: FlowData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EnvType {
    // 基本（引擎相关）
    Base,
    // 运行时（系统运行时，如Java、Python）
    Runtime,
    // 插件（引擎扩展插件）
    Plugin,
    // 扩展（功能扩展）
    Extend,
    // 运载服务（运行时服务，如rust、db等）
    Service,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Environment {
    pub name: String,
    pub env_type: EnvType,
    pub version: String,
}

// 流程数据
#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode, Default)]
pub struct FlowData {
    // 系统参数域，不要手动在代码里对其修改，属于系统自带的变量
    pub basics: SystemFlowData,
    // 用户参数域，可以理解为声明的变量
    pub params: HashMap<String, String>,
    // 节点数据域，有的节点会将处理结果放到此处
    pub nodes: HashMap<String, Vec<u8>>,
    // 数据统一为二进制，使用时需要根据具体情况判断
    pub data: HashMap<String, Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode, Default)]
pub struct SystemFlowData {
    // 下游数据，一般由逻辑节点控制，用于控制节点的跳转
    pub downstream: Vec<String>,
    // 最大重复次数，默认为可索引节点数量 + 10，每执行一个节点，会使此数量-1，超出后强制停止流的执行
    // 可以有效避免死循环的产生，如果设置为-1，则不会对执行次数进行限制
    pub maximum_repetition: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode)]
pub struct SubFlowTransferData {
    pub nodes: Vec<Node>,
    pub flow_data: FlowData,
}

// logical block
#[derive(Serialize, Deserialize, Clone, Debug, Encode, Decode, Default)]
pub struct Blueprint {
    pub parallel_endpoints: bool,
    pub parallel_routes: bool,
    #[serde(default = "default_maximum_repetition")]
    pub maximum_repetition: i32,
    pub endpoints: Vec<String>,
    pub routes: HashMap<String, Node>,
}

fn default_maximum_repetition() -> i32 { 99 }
