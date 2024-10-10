use crate::core::flow::controller::interface::check_require;
use crate::core::flow::exec::node::exec_node;
use crate::core::flow::resolver::interface::flow_resolver;
use engine_common::entity::error::{DispatchErr, NodeError};
use engine_common::entity::flow::{Flow, FlowData, FlowRuntimeModel, FlowStatus, Node, NodeTag, SystemFlowData};
use engine_common::logger::interface::{fail, info, success, warn};
use engine_common::runtime::flow::{get_flow_runtime, set_flow_runtime};
use std::path::Path;
use std::string::String;

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
        let mut node = node.clone();
        node.id = Some(endpoint.as_str().to_string());
        // 执行节点
        match dispatch_nodes(flow.clone(), node, &mut runtime.data.clone()).await {
            Ok(_) => {}
            Err(e) => {
                return flow_dispatch_err_handler(e)
            }
        }
    }
    success(format!("flow {{ {} }} has be exec success.", flow.name).as_str());
    Ok(())
}

async fn dispatch_nodes(flow: Flow, current_node: Node, mut data: &mut FlowData) -> Result<(), DispatchErr> {
    if data.basics.maximum_repetition != -1 {
        if data.basics.maximum_repetition > 0 {
            data.basics.maximum_repetition -= 1;
        } else if data.basics.maximum_repetition <= 0 {
            // 直接跳出调用
            return Err(DispatchErr::RunOverLimit);
        }
    }
    let c_flow = flow.clone();

    let downstream: Vec<String>;
    let current_data = data.clone();

    match exec_node(current_node.clone(), &mut data).await {
        Ok(_) => {}
        Err(e) => {
            // 根据流节点配置或系统默认配置决定下一步操作
            // 如果返回的是false，将终止流的执行
            if !node_expect_dispose(e) {
                return Err(DispatchErr::FlowFailed("Node execution failed".to_string()))
            }
            if current_node.redress_stream.is_some() {
                // 这部分需要根据配置进行，可以分线程或阻塞进行
                let redress_stream = current_node.redress_stream.unwrap();
                for stream_id in redress_stream {
                    let stream = c_flow.blueprint.routes.get(&stream_id).expect("cannot find stream in router.");
                    let mut stream = stream.clone();
                    stream.id = Some(stream_id.as_str().to_string());
                    // 尝试执行补偿流
                    match Box::pin(dispatch_nodes(c_flow.clone(), stream.clone(), data)).await {
                        Ok(_) => {}
                        Err(_) => {
                            // 流执行失败
                            return Err(DispatchErr::RedressFailed);
                        }
                    }
                }
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
        return Ok(());
    }
    Ok(for node_id in downstream {
        let node = flow.blueprint.routes.get(&node_id).expect("cannot find endpoint in router.");
        let mut node = node.clone();
        node.id = Some(node_id.as_str().to_string());
        // 将递归调用的结果装箱
        match Box::pin(dispatch_nodes(c_flow.clone(), node.clone(), data)).await {
            Ok(_) => {}
            Err(_) => {
                // 流执行失败
                return Err(DispatchErr::FlowFailed("Recursive processing of blueprints times errors".to_string()));
            }
        }
    })
}

// 节点异常统一处理
// 如果返回了false，将断开流的执行
fn node_expect_dispose(node_err: NodeError) -> bool {
    match node_err {
        NodeError::ExtNotFound(ext) => {
            fail(format!("extension {} could not be found.", ext).as_str());
            // TODO: 根据配置决定是否要退出执行
            return false;
        }
        // 扩展中的方法执行失败
        NodeError::ExtError(ext) => {
            fail(format!("extension {} method execution failed.", ext).as_str());
            return false;
        }
        NodeError::HandleRuntimeError(_) => {}
        NodeError::HandleNotFound(_) => {}
        NodeError::RouteError(_) => {}
        NodeError::ParamNotFound(_) => {}
        NodeError::ParamFormatError(_) => {}
        NodeError::ParamParseError(_) => {}
        NodeError::PathNotFound => {}
        NodeError::PathCreateError => {}
        NodeError::PathDeleteError => {}
        NodeError::PathMoveError(_) => {}
        NodeError::PathCopyError => {}
        NodeError::PathChmodError => {}
        NodeError::PathOtherError(_) => {}
        NodeError::FileNotFound => {}
        NodeError::FileTypeError => {}
        NodeError::FileReadError => {}
        NodeError::FileWriteError(_) => {}
        NodeError::FileCreateError => {}
        NodeError::FileDeleteError => {}
        NodeError::FileMoveError => {}
        NodeError::FileCopyError => {}
        NodeError::FileChmodError => {}
        NodeError::FileOtherError(_) => {}
        NodeError::RequirePermission => {}
        NodeError::ScriptExecError(_) => {}
        NodeError::ScriptExecTimeout => {}
        NodeError::ScriptExecFailed => {}
        NodeError::ScriptExecRejected => {}
        NodeError::NetworkUrlNotFound => {}
        NodeError::NetworkConnectError => {}
        NodeError::NetworkRequestError => {}
        NodeError::NetworkResponseError => {}
        NodeError::NetworkTimeoutError => {}
        NodeError::NetworkRejectedError => {}
        NodeError::NetworkOtherError(_) => {}
    }
    true
}

// 流调度错误统一处理器
fn flow_dispatch_err_handler(err: DispatchErr) -> Result<(), DispatchErr> {
    match err {
        DispatchErr::FlowFailed(_) => { Ok(()) }
        DispatchErr::RedressFailed => { Ok(()) }
        DispatchErr::RunOverLimit => { Ok(()) }
        _ => { Ok(()) }
    }
}