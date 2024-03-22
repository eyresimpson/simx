use std::path::Path;
use std::process::Command;

use crate::conf::simx::get_config;
use crate::tools::log::shell::{err, info, script_err, script_log, warn};

pub fn exec_python_script(path: &Path) {
    println!("sss: {:?}", path);
    let conf = get_config();
    let python = conf.get("python");
    if !python.unwrap().get("enable").unwrap().as_bool().unwrap() {
        return;
    }
    let file_name = path.file_name().unwrap().to_str().unwrap_or_default();
    info(format!("Find python in path -> {}", file_name).as_str());
    // let command = "";
    let exec_path: String;
    // 优先使用py3，除非禁用了py3（其实本来压根就不想兼容py2，后续会去掉对py2的支持）
    if python.unwrap().get("python3-enable").unwrap().as_bool().unwrap() == true {
        exec_path = python.unwrap().get("python3-path").unwrap().as_str().unwrap().parse().unwrap();
    } else if python.unwrap().get("python2-enable").unwrap().as_bool().unwrap() == true {
        warn("Warning, simx will soon remove support for python2 in a later version.");
        exec_path = python.unwrap().get("python2-path").unwrap().as_str().unwrap().parse().unwrap();
    } else {
        err("All versions of python execution are disabled in the configuration file, and you must enable any of them.");
        return;
    }
    let output = Command::new(exec_path)
        .arg(path)
        .output()
        .expect("Failed to execute py script");
    // 输出执行结果
    if &*String::from_utf8_lossy(&output.stdout) != "" {
        script_log(&*String::from_utf8_lossy(&output.stdout).trim());
    }
    if &*String::from_utf8_lossy(&output.stderr) != "" {
        script_err(&*String::from_utf8_lossy(&output.stderr).trim());
    }
}
