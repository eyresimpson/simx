use rocket::{build, routes, Config};

pub fn start_net_watcher() {
    let future = async {
        build().configure(Config::default()).mount("/simx", routes![
        ]).launch().await.expect("Cannot load rest services.");
    };

    tokio::runtime::Runtime::new().unwrap().block_on(future);
}