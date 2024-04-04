use std::fs::File;
use std::io::Read;
use std::path::Path;

use toml::Value;

// 加载配置文件
pub fn load_conf(file_path: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path).unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;
    let toml = toml_str.parse::<Value>().unwrap();
    Ok(toml)
}
