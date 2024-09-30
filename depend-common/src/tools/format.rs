// 将Json转换为u8进行存储
pub fn json_to_u8(json: String) -> Vec<u8> {
    json.into_bytes()
}

// 将u8转换为Json
pub fn u8_to_json(u8: Vec<u8>) -> String {
    String::from_utf8(u8).unwrap()
}