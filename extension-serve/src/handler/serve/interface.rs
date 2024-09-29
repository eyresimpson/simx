use crate::handler::serve::http::interface::start_net_watcher;

pub fn serve() {
    let future = async {
        start_net_watcher().await;
    };
    println!("Simx serve has started.");
    tokio::runtime::Runtime::new().unwrap().block_on(future);
}