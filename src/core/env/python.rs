use toml::Value;

pub fn check_python(conf: Value) -> bool {
    let current_python = conf.get("python").unwrap().get("path").unwrap().as_str().unwrap();
    // 注意！ Windows 10/11即使没有安装，也不会报错（因为微软不知道怎么想得，如果未安装居然会默认打开应用商店...）
    std::process::Command::new(current_python).arg("--version").output().is_ok()
}
