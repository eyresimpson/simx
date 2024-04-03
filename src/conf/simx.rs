use std::path::Path;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use toml::Value;

use crate::tools::files::toml::load_conf;

// 禁止直接操作静态配置
static CONFIG: Lazy<Mutex<Value>> = Lazy::new(|| {
    Mutex::new(load_conf(Path::new("conf").join("simx.toml").as_path()).unwrap())
});

static NET_CONFIG: Lazy<Mutex<Value>> = Lazy::new(|| {
    Mutex::new(load_conf(Path::new("conf").join("net.toml").as_path()).unwrap())
});

static ENV_CONFIG: Lazy<Mutex<Value>> = Lazy::new(|| {
    Mutex::new(load_conf(Path::new("conf").join("env.toml").as_path()).unwrap())
});


// 获取配置
pub fn get_engine_config() -> Value {
    CONFIG.lock().unwrap().clone()
}

// 获取网络配置
pub fn get_net_conf() -> Value {
    NET_CONFIG.lock().unwrap().clone()
}

// 获取环境配置
pub fn get_env_conf() -> Value {
    ENV_CONFIG.lock().unwrap().clone()
}

