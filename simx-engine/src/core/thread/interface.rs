use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// 初始化线程池
pub fn init_thread_pool() {
    // 创建通道用于线程之间的消息传递
    let (tx_a, rx_a) = mpsc::channel();
    let (tx_b, rx_b) = mpsc::channel();

    // 用户定义的线程池大小
    let pool_size = 4;

    // 使用Arc和Mutex来控制线程池的线程数量
    let pool = Arc::new(Mutex::new(threadpool::ThreadPool::new(pool_size)));

    let pool_a = Arc::clone(&pool);
    let pool_b = Arc::clone(&pool);

    // 创建线程a
    thread::spawn(move || {
        for msg in rx_a {
            let pool_a = Arc::clone(&pool_a);
            pool_a.lock().unwrap().execute(move || {
                println!("Thread A received: {}", msg);
            });
        }
    });

    // 创建线程b
    thread::spawn(move || {
        for msg in rx_b {
            let pool_b = Arc::clone(&pool_b);
            pool_b.lock().unwrap().execute(move || {
                println!("Thread B received: {}", msg);
            });
        }
    });

    // 主线程向线程a和线程b发送消息
    tx_a.send("Hello from main to A").unwrap();
    tx_b.send("Hello from main to B").unwrap();

    // 给线程一些时间来处理消息
    thread::sleep(std::time::Duration::from_secs(1));
}