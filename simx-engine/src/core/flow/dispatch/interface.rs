use crate::core::flow::controller::interface::check_require;
use crate::core::flow::exec::node::exec_node;
use crate::core::flow::resolver::interface::flow_resolver;
use engine_common::entity::flow::{Flow, FlowData, FlowRuntimeModel, FlowStatus, Node, NodeTag, SystemFlowData};
use engine_common::logger::interface::{fail, info, success, warn};
use engine_common::runtime::flow::{get_flow_runtime, set_flow_runtime};
use std::path::Path;
use std::string::String;

// 调度执行流
// 此方法会根据流文件的path或json，生成Flow运行时并调度执行
// 第一个参数是流文件路径，第二个是流文件内容，两个参数必须有一个不为空
// 建议在新线程中执行此方法
pub async fn dispatch_flow(path: &Path) {
    // 流对象
    let mut flow: Flow;

    // 在缓存中搜索路径是否存在，如果存在就不再走文件系统
    let test_cache = get_flow_runtime(path.to_str().unwrap());

    // TODO：这个判断并不稳定，有可能flow会发生改变，因此需要监视该文件，这个是后续的功能
    if test_cache.is_some() {
        flow = test_cache.unwrap();
    } else {
        // 尝试解析流文件
        if path.exists() && path.is_file() {
            // 加载流文件并解析为Flow对象
            flow = flow_resolver(path);
            // 将当前流加入到缓存
            set_flow_runtime(path.to_str().unwrap(), flow.clone());
        } else {
            fail("cannot find or open flow file.");
            return;
        }
    }

    // 检查流文件的环境要求
    check_require(flow.clone().requirements).expect("Check flow require failed!");

    info(format!("flow {{ {} }} will be exec.", flow.name).as_str());
    // 创建流运行时
    flow.runtime = Some(FlowRuntimeModel {
        status: FlowStatus::Starting,
        history: Default::default(),
        logs: vec![],
        current_node: None,
        data: FlowData {
            basics: SystemFlowData {
                downstream: vec![],
                maximum_repetition: flow.blueprint.maximum_repetition,
            },
            params: Default::default(),
            nodes: Default::default(),
            data: Default::default(),
        },
    });

    let runtime = flow.clone().runtime.unwrap();
    // 取入口节点群并尝试执行
    let endpoints = flow.clone().blueprint.endpoints;
    for endpoint in endpoints {
        // 根据入口节点id获取节点对象
        let node = flow.blueprint.routes.get(&endpoint).expect("cannot find endpoint in router.");
        // 执行节点
        dispatch_nodes(flow.clone(), node.clone(), &mut runtime.data.clone()).await;
    }

    success(format!("flow {{ {} }} has be exec success.", flow.name).as_str());
}

async fn dispatch_nodes(flow: Flow, current_node: Node, mut data: &mut FlowData) {
    if data.basics.maximum_repetition != -1 {
        if data.basics.maximum_repetition > 0 {
            data.basics.maximum_repetition -= 1;
        } else if data.basics.maximum_repetition <= 0 {
            // 直接跳出调用
            return;
        }
    }
    let c_flow = flow.clone();

    let downstream: Vec<String>;
    let ret = exec_node(current_node.clone(), &mut data).await;
    let current_data = data.clone();

    // 判断执行结果，决定是否执行补偿流
    if ret.is_err() {
        // TODO: 根据流节点配置或系统默认配置决定下一步操作

        if current_node.redress_stream.is_some() {
            let redress_stream = current_node.redress_stream.unwrap();
            for stream_id in redress_stream {
                let stream = c_flow.blueprint.routes.get(&stream_id).expect("cannot find stream in router.");
                // 尝试执行补偿流
                Box::pin(dispatch_nodes(c_flow.clone(), stream.clone(), data)).await;
            }
        }
    }

    // 如果节点为Route类型的节点，就从节点参数中取新的downstream
    if current_node.tags.is_some() && current_node.tags.unwrap().contains(&NodeTag::Route) {
        if current_data.basics.downstream.is_empty() {
            warn("The logical node does not process the downstream link, downstream is empty, Skip...");
            // 相对于直接结束了流的运行
            downstream = vec![];
        } else {
            // 将新的downstream赋值给流数据
            downstream = current_data.basics.downstream;
        }
        // 主动清空路由节点的下游数据，防止影响到后续节点的执行
        data.basics.downstream.clear();
    } else {
        downstream = current_node.downstream;
    }
    if downstream.is_empty() {
        return;
    }
    for node_id in downstream {
        let node = flow.blueprint.routes.get(&node_id).expect("cannot find endpoint in router.");
        // 将递归调用的结果装箱
        Box::pin(dispatch_nodes(c_flow.clone(), node.clone(), data)).await;
    }
}