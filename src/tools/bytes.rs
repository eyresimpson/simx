pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8(Vec::from(bytes)).unwrap()
}

pub fn string_to_bytes(string: String) -> Vec<u8> {
    string.into_bytes()
}