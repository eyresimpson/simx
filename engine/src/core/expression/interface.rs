use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

// 最终生产一个string表达式，可以解析表达式中的变量
pub fn preliminary_analysis_string(expr: String, value: HashMap<String, Value>) -> String {
    // 解析表达式中的变量符号
    replace_placeholders(expr.as_str(), value)
}

// #[test]
// fn test_replace_placeholders() {
//     let input = "这是一个测试字符串，包含 ${aaa} 和 ${bbb}。";
//     let expected = "这是一个测试字符串，包含 123 和 456。";
//     assert_eq!(preliminary_analysis_string(input, "123456"), expected);
// }

fn replace_placeholders(input: &str, dist: HashMap<String, Value>) -> String {
    let re = Regex::new(r"\$\{(\w+)\}").unwrap();
    re.replace_all(input, |caps: &regex::Captures| {
        match dist.get(&caps[1]) {
            Some(value) => value.as_str().unwrap(),
            None => "",
        }
    }).to_string()
}