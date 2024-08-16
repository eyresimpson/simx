use crate::handler::serve::http::interface::start_net_watcher;

pub fn serve() {
    let future = async {
        start_net_watcher().await;
    };

    tokio::runtime::Runtime::new().unwrap().block_on(future);
}