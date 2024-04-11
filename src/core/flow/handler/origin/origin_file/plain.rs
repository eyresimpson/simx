use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn handle_origin_file_plain(handler_str: String, data: HashMap<String, String>, args: HashMap<String, String>){
    // file_reader("".to_string());
}

// 此方法仅会触发一次，不是一个持续性的任务
fn file_reader(file_path: String) -> String {
    let read_file_ret = File::open(file_path);
    let mut file = read_file_ret.unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    return toml_str;
}

// 此方法会持续监听文件变化，文件变动后触发流
fn file_listener(){

}