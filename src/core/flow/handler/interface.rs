use std::collections::HashMap;
use crate::core::common::log::shell::warn;
use crate::core::flow::entity::standardisation::{Data, Node};


pub fn handler(node: Node, flow_data: &Data) {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    // 判断是否为内置 handler，内置的handler必然以simx开头
    if handler_path[0] == "simx" {
        // 如果是内置的handler
        println!("-----> {:?}", handler_path[handler_path.len() - 1]);
        match handler_path[1] {
            "files" => {
                // flow_data = handle_origin_file_plain(flow_data.clone(), step.attr);
            }
            "println" => {
                // flow_data = handle_origin_file_plain(flow_data.clone(), step.attr);
            }
            "writer" => {
                // flow_data = handle_origin_file_plain(flow_data.clone(), step.attr);
            }

            _ => {
                warn("Engine cannot find handle function, Skip...");
            }
        }
    } else {
        // 如果不是内置的handler
    }
    // for step in steps {
    //     match step.func.as_str() {
    //         "file_reader" => {
    //             // flow_data = handle_origin_file_plain(flow_data.clone(), step.attr);
    //         }
    //
    //         _ => {
    //             warn("Engine cannot find handle function, Skip...")
    //         }
    //     }
    // }
    return;
}
