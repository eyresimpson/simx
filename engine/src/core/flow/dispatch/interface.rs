use crate::core::flow::controller::interface::check_require;
use crate::core::flow::dispatch::common::redress_stream_dispatch;
use crate::core::flow::dispatch::dispatch_general::dispatch_general;
use crate::core::flow::dispatch::dispatch_loop::dispatch_loop;
use crate::core::flow::resolver::interface::flow_resolver;
use engine_common::entity::error::{DispatchErr, NodeError};
use engine_common::entity::flow::{Blueprint, Flow, FlowData, FlowRuntimeModel, FlowStatus, Node, NodeTag, SystemFlowData};
use engine_common::logger::interface::{fail, info, success};
use engine_common::runtime::flow::{get_flow_runtime, set_flow_runtime};
use std::path::Path;
use engine_common::exception::flow::flow_dispatch_err_handler;

// 流调度执行器
// 此方法会根据流文件的path，生成Flow运行时并调度执行
pub async fn dispatch_flow(path: &Path) -> Result<(), DispatchErr> {
    // 流对象
    let mut flow: Flow;

    // 在缓存中搜索路径是否存在，如果存在就不再走文件系统
    // TODO：这个判断并不稳定，有可能flow会发生改变，因此需要缓存文件修改日期进行比较，这个是后续的功能
    match get_flow_runtime(path.to_str().unwrap()) {
        Some(f) => {
            // 直接使用缓存中的数据
            flow = f;
        }
        None => {
            // 尝试解析流文件
            if path.exists() && path.is_file() {
                // 加载流文件并解析为Flow对象
                flow = flow_resolver(path);
                // 将当前流加入到缓存
                set_flow_runtime(path.to_str().unwrap(), flow.clone());
            } else {
                fail("cannot find or open flow file.");
                return Err(DispatchErr::FlowNotFound(path.to_str().unwrap().to_string()));
            }
        }
    }


    // 检查流文件的环境要求
    match check_require(flow.clone().requirements) {
        Ok(_) => {}
        Err(e) => {
            fail(e.as_str());
            return Err(DispatchErr::RequireError(e));
        }
    }

    info(format!("flow {{ {} }} will be exec.", flow.name).as_str());
    // 创建流运行时
    flow.runtime = Some(FlowRuntimeModel {
        status: FlowStatus::Starting,
        current_node: None,
        data: FlowData {
            basics: SystemFlowData {
                route: Default::default(),
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
        let mut node = node.clone();
        node.id = Some(endpoint.as_str().to_string());
        // 执行节点
        match dispatch_nodes(flow.blueprint.clone(), node, &mut runtime.data.clone()).await {
            Ok(_) => {}
            Err(e) => {
                return flow_dispatch_err_handler(e)
            }
        }
    }
    success(format!("flow {{ {} }} has be exec success.", flow.name).as_str());
    Ok(())
}

// 调度节点（蓝图）
// 蓝图、首节点（从这个节点开始执行）、数据
pub async fn dispatch_nodes(blueprint: Blueprint, current_node: Node, data: &mut FlowData) -> Result<(), DispatchErr> {
    // 当前节点的标签列表
    let tags = match current_node.tags {
        None => { Vec::new() }
        Some(ref tags) => { tags.clone() }
    };
    // 当前节点是否为loop
    let is_loop = tags.contains(&NodeTag::Loop);
    // 是否为Jump节点
    let is_jump = tags.contains(&NodeTag::Jump);
    // Loop节点、Jump节点、Opt节点（操作节点）
    // Loop、Jump、Opt不参与exec_node调度，直接在此处实现
    if !current_node.handler.is_empty() {
        // 作为普通节点进行调度
        dispatch_general(blueprint, current_node, data).await
    } else if is_loop {
        match Box::pin(dispatch_loop(blueprint.clone(), current_node.clone(), data)).await {
            Ok(_) => { Ok(()) }
            Err(_) => {
                fail("The implicated compensation mechanism is triggered".to_string().as_str());
                // 执行当前节点的Redress_stream，如果节点报错，会依次执行之前所有节点的Redress_stream
                return redress_stream_dispatch(NodeError::Redress("The implicated compensation mechanism is triggered".to_string()), &current_node, &blueprint, data).await;
            }
        }
    } else if is_jump {
        // 作为Jump节点进行处理
        Ok(())
    } else { Ok(()) }
}


