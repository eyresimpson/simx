use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use toml::Value;

// 禁止直接操作静态配置
// TODO：需要在此处添加一些判断机制，如果未成功加载正常报错
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

// 加载配置文件
pub fn load_conf(file_path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let read_file_ret = 
    let mut file = File::open(file_path).unwrap();

    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;
    let toml = toml_str.parse::<Value>().unwrap();
    Ok(toml)
}
