use uuid::Uuid;

pub fn getUuid() -> String {
    Uuid::new_v4().to_string()
}

// 获取当前时间
pub fn getCurrentTime(format: &str) -> String {
    let now = chrono::Local::now();
    now.format(format).to_string()
}