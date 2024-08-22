use crate::core::flow::exec::flow::exec_steps;
use crate::core::flow::interface::exec_flow;
use crate::core::script::interface::exec_script;
use engine_common::entity::flow::SubFlowTransferData;
use engine_common::entity::simx::SimxThreadSenderStringData;
use engine_common::logger::interface::warn;
use engine_common::runtime::thread::set_engine_sender;
use std::sync::mpsc;
use std::thread;
use tokio::runtime::Builder;

pub fn init_thread_monitor() {
    // 使用 tokio::runtime::Builder 创建一个自定义的运行时
    let rt = Builder::new_multi_thread()
        // 设置最大工作线程数为 4
        .worker_threads(4)
        // 设置最大阻塞线程数为 8
        .max_blocking_threads(8)
        // 启用所有 Tokio 的功能（如定时器、IO 等）
        .enable_all()
        .build()
        .unwrap();

    // 创建通道用于线程之间的消息传递
    let (engine_sender, engine_receiver) = mpsc::channel::<SimxThreadSenderStringData>();

    // 创建引擎监听线程
    thread::spawn(move || {
        rt.block_on(async {
            for msg in engine_receiver {
                // 使用tokio的异步任务执行器来执行异步任务
                tokio::spawn(async move {
                    // 匹配线程消息对象
                    match msg.function.as_str() {
                        "exec_flow" => {
                            exec_flow(msg.data.as_ref()).await;
                        }
                        "exec_script" => {
                            exec_script(msg.data.as_ref());
                        }
                        "exec_steps" => {
                            // 序列化出对象
                            let transfer = serde_json::from_str::<SubFlowTransferData>(msg.data.as_ref()).unwrap();
                            exec_steps(transfer.nodes, transfer.flow_data);
                        }
                        _ => {
                            warn("Unparsable thread message function!");
                        }
                    }
                });
            }
        });
    });

    set_engine_sender("engine_sender", engine_sender);

    // 给线程一些时间来处理消息
    // thread::sleep(std::time::Duration::from_secs(1));
}