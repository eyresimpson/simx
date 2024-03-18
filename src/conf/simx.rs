pub use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use toml::Value;

pub static CONFIG: Lazy<Mutex<Value>> = Lazy::new(|| {
    Mutex::new(load_conf("config/simx.toml").unwrap())
});

// 加载配置文件
pub fn load_conf(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;
    let toml = toml_str.parse::<Value>().unwrap();

    Ok(toml)
}

pub fn get_config() -> Value {
    CONFIG.lock().unwrap().clone()
}
