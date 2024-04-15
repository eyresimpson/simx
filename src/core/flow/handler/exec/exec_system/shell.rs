use std::collections::HashMap;

// 控制台/命令行相关（如打印）
pub fn handle_exec_system_shell(data: HashMap<String, String>, step_args: HashMap<String, String>) -> HashMap<String, String> {
    println!("{}", step_args.get("text").unwrap());
    return data;
}