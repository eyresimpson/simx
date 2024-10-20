use engine_common::entity::flow::flow::{FlowData};
use engine_common::entity::flow::node::Node;
use engine_common::logger::interface::warn;

pub async fn handle_net_http(node: Node, flow_data: &mut FlowData) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[3] {
        // 发起Get请求(同步)
        // 系统会等待此方法执行完毕
        "request_get" => request_get(node, flow_data).await,

        // 发起Post请求
        "request_post" => request_post(node, flow_data),
        _ => {
            warn(format!("Engine cannot find handler string by {}, Skip...", handler_path[3]).as_str());
        }
    }
}

// 发起Get请求
async fn request_get(node: Node, flow_data: &mut FlowData) {
    println!("handle http get: {:?},{:?}", node, flow_data);
    // let rest = reqwest::get("https://httpbin.org/ip").await;
    // if rest.is_err() {
    //     fail("Cannot launch new http get request.");
    // } else {
    //     // flow_data.data.insert("res".to_string(), rest.unwrap().);
    //     println!("Res: {:?}", rest);
    // }
    // 获取请求地址
    // let addr = node.attr.get("addr").unwrap();
    // // 获取请求端口
    // let port = node.attr.get("port").unwrap();
    // // 获取请求参数
    // let params = node.attr.get("params").unwrap();
    // // 获取请求头
    // let header = node.attr.get("header").unwrap();
    // let map = get("").await.unwrap();
    // let ret = to_string(&map).unwrap();
    // println!("===> {}", ret);
    // flow_data.data.insert("res".to_string(), ret);
}

// 发起Post请求
fn request_post(node: Node, flow_data: &FlowData) {
    println!("{:?},{:?}", node, flow_data)
}

// // 发起Put请求
// fn request_put(node: Node, flow_data: &FlowData) {
//     println!("{:?},{:?}", node, flow_data)
// }
//
// // 发起Delete请求
// fn request_delete(node: Node, flow_data: &FlowData) {
//     println!("{:?},{:?}", node, flow_data)
// }
