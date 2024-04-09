use std::path::Path;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use toml::Value;

use crate::conf::toml::load_conf;

// 运行时配置对象
static RUNTIME: Lazy<Mutex<Value>> = Lazy::new(|| {
    Mutex::new(load_conf(Path::new("conf").join("simx.toml").as_path()).unwrap())
});

// 运行时配置
pub fn get_runtime_conf() -> Value {
    crate::conf::runtime::RUNTIME.lock().unwrap().clone()
}
