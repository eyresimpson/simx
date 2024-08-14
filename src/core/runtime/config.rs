use std::fs;
use std::path::Path;
use std::sync::Mutex;

// 运行时配置
use once_cell::sync::Lazy;

use crate::entity::engine::{EngineConfig, EnvConfig, NetConfig, SimxConfig};

// 此处用于加载配置文件，并解析为结构体，注意配置一般不允许动态修改

// 全局静态变量
pub static CONFIG: Lazy<Mutex<SimxConfig>> = Lazy::new(|| {
    // 判断是否有配置，现在允许无配置的情况下运行
    if !Path::new("conf").join("simx.toml").exists() {
        let config: SimxConfig = SimxConfig{
            engine: EngineConfig::default(),
            net: NetConfig::default(),
            env: EnvConfig::default()
        };
        Mutex::new(config)
    } else {
        let config_path = Path::new("conf").join("simx.toml");
        let config_str = fs::read_to_string(config_path).unwrap_or_else(|_| String::new());
        let config: SimxConfig = toml::from_str(&config_str).unwrap_or_default();
        Mutex::new(config)
    }
});

pub fn get_simx_config() -> SimxConfig {
    CONFIG.lock().unwrap().clone()
}