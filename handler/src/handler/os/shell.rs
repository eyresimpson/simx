use engine_common::entity::error::NodeError;
use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::success;
use engine_common::tools::format::u8_to_str;

// 控制台/命令行相关（如打印）
pub fn handle_os_shell_println(node: Node, _flow_data: &mut FlowData) -> Result<(), NodeError> {
    match node.attr.get("command") {
        Some(command) => {
            println!("{:?}", command);
            return Ok(())
        }
        None => {}
    }
    // 获取命令名称
    if !node.attr.contains_key("command") {
        return Err(NodeError::ParamNotFound("command".to_string()))
    }
    let name = node.attr.get("command").unwrap();
    let name = u8_to_str(name.to_vec());
    // 获取命令参数列表
    let mut command = std::process::Command::new(name);
    if node.attr.contains_key("args") && node.attr.get("args").unwrap().len() > 0 {
        match node.attr.get("args") {
            Some(args) => {
                let args = u8_to_str(args.to_vec());
                let args: Vec<String> = args.split(" ").map(|s| s.to_string()).collect();
                for arg in args {
                    command.arg(arg);
                }
            }
            None => return Err(NodeError::ParamNotFound("args".to_string())),
        }
        
    }
    let result = command.output();
    if !result.is_err() {
        let result = result.unwrap();
        let vec: Vec<u8>;
        // 就非常离谱，为啥正常执行的结果会输出到err里。。。
        if result.stdout.is_empty() {
            vec = result.stderr;
        } else {
            vec = result.stdout;
        }
        success(format!("{:?}", String::from_utf8_lossy(&vec)).as_str());
        Ok(())
    } else {
        Err(NodeError::HandleRuntimeError(result.err().unwrap().to_string()))
    }
}
