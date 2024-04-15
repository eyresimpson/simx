use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use crate::core::common::log::shell::{err, warn};

pub fn handle_endpoint_file_plain(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    file_writer(data.clone(), args.clone())
}

fn file_writer(data: HashMap<String, String>, args: HashMap<String, String>) -> HashMap<String, String> {
    let file_path = args.get("path").unwrap();
    let file_content = args.get("content").unwrap();
    let writer_type = args.get("type").unwrap();

    // if File:: {  }
    let mut file = File::open(file_path);
    if file.is_err() {
        warn("Cannot find specified file, will create it.");
        file = File::create(file_path);
    }
    let ret = file.unwrap().write(file_content.as_ref());
    if ret.is_err() {
        err("Cannot write content to file.");
        err(ret.err().unwrap().to_string().as_str());
    }
    return data;
}
