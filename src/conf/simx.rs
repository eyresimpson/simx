use std::path::Path;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use toml::Value;

use crate::tools::files::toml::load_conf;

// 禁止直接操作静态配置
static CONFIG: Lazy<Mutex<Value>> = Lazy::new(|| {
    Mutex::new(load_conf(Path::new("config").join("simx.toml").as_path()).unwrap())
});


// 获取配置
pub fn get_config() -> Value {
    CONFIG.lock().unwrap().clone()
}
