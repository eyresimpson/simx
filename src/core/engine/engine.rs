use serde_json::from_str;

use crate::conf::runtime::get_runtime_conf;
use crate::core::common::log::interface::{fail, info, success};
use crate::core::engine::init::engine_init;
use crate::core::engine::watcher::start_net_watcher;
use crate::core::extension::dll::interface::call_extension_method;
use crate::entity::ext::Extension;

/// 引擎核心
/// 其实引擎启动主要是启动了系统监听，引擎本身并不会持续运行，否则会占用一些不必要的资源，当有请求抵达监听器时，
/// 才会调用引擎方法，发起流程或脚本
pub async fn run() {
    info("Engine Starting...");

    // 执行系统初始化事件
    // 包括运行初始化脚本和初始化流
    let init_ret = engine_init().await;
    if init_ret.is_err() {
        fail(init_ret.err().unwrap().as_str());
        return;
    }

    // test: 调用
    info("Test: Calling...");
    // 查询数据库
    // ;
    let extension: Extension = from_str(get_runtime_conf("core").unwrap().as_str()).unwrap();
    let functions = extension.function.get(0).unwrap();

    call_extension_method("D:\\Code\\simx-core\\release\\core.dll".to_string(), functions.clone());
    // extension
    // 从配置中获取数据库路径
    // let db_path = get_runtime_conf("db_path").unwrap();
    // // 链接到数据库
    // let conn = Connection::open(format!("{}/simx.db", db_path)).unwrap();
    // let sql = "SELECT * FROM simx_ext WHERE id = (?1)";
    // let mut stmt = conn.prepare(sql).unwrap();
    // let mut rows = stmt.query(params![1]).unwrap();
    // 
    // while let Some(row) = rows.next().unwrap() {
    //     let funcs: String = row.get(6).unwrap();
    //     let extension = Extension {
    //         name: row.get(1).unwrap(),
    //         version: row.get(2).unwrap(),
    //         description: row.get(3).unwrap(),
    //         license: row.get(4).unwrap(),
    //         author: row.get(5).unwrap(),
    //         keywords: vec![],
    //         dependencies: vec![],
    //         function: from_str(&funcs).unwrap(),
    //     };
    // 
    //     // 可以根据需要处理 extension
    //     println!("{:?}", extension);
    // }


    // 系统启动完成
    success("Engine has started.");

    // 尝试调起网络监听器（阻塞）
    start_net_watcher().await;

    // 运行结束
    // 如果是用户手动结束进程，不会执行到这里（只有系统主动结束此处才会执行）
    success("Engine run out.");
}
