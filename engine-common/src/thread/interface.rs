use crate::entity::simx::{SimxThreadFunctions, SimxThreadSenderStringData};
use crate::runtime::thread::set_sender_info;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// 初始化线程池
pub fn init_thread_monitor(simx_thread_functions: SimxThreadFunctions) {
    // 使用Arc和Mutex来控制线程池的线程数量
    let pool = Arc::new(Mutex::new(threadpool::ThreadPool::new(15)));

    let exec_flow = simx_thread_functions.exec_flow;
    let exec_script = simx_thread_functions.exec_script;
    // 创建通道用于线程之间的消息传递
    let (engine_sender, engine_receiver) = mpsc::channel::<SimxThreadSenderStringData>();

    // 创建引擎监听线程
    thread::spawn(move || {
        for msg in engine_receiver {
            pool.lock().unwrap().execute(move || {
                match msg.function.as_str() {
                    "exec_flow" => {
                        exec_flow(msg.data.as_ref())
                    }
                    "exec_script" => {
                        // 脚本也会在新线程中执行
                        exec_script(msg.data.as_ref())
                    }
                    _ => {}
                }
            });
        }
    });

    set_sender_info("engine_sender", engine_sender);
    
    // 给线程一些时间来处理消息
    thread::sleep(std::time::Duration::from_secs(1));
}