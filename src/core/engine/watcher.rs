use crate::core::watcher::http::start_http_watcher;
use crate::tools::log::shell::info;

pub fn start_net_watcher() {
    info("Engine Net Services Starting...");
    start_http_watcher();
}

