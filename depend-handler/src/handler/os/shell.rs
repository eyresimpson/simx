use engine_common::entity::flow::{FlowData, Node};
use engine_common::logger::interface::{fail, success};

// 控制台/命令行相关（如打印）
pub fn handle_os_shell_println(node: Node, _flow_data: &mut FlowData) {
    // 获取命令名称
    if !node.attr.contains_key("command") {
        fail("Command name not found.");
        return;
    }
    let name = node.attr.get("command").unwrap();
    // 获取命令参数列表
    let mut command = std::process::Command::new(name);
    if node.attr.contains_key("args") && node.attr.get("args").unwrap().len() > 0 {
        let args: Vec<String> = node.attr.get("args").unwrap().to_string().split(" ").map(|s| s.to_string()).collect();
        for arg in args {
            command.arg(arg);
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
    } else {
        fail(format!("{:?}", result.err()).as_str())
    }
}
