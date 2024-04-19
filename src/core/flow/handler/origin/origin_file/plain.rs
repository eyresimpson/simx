use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn handle_origin_file_plain(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    file_reader(data.clone(), args.clone())
}

// 此方法仅会触发一次，不是一个持续性的任务
fn file_reader(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    let file_path = args.get("path").unwrap();
    // 返回的数据
    let mut ret: HashMap<String, String> = data;
    let read_file_ret = File::open(file_path);
    let mut file = read_file_ret.unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    ret.insert("text".parse().unwrap(), toml_str);
    return ret;
}