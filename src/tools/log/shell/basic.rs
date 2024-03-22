
pub fn info(text: &str) {
    println!("➜ Info: {}", text);
}

pub fn success(text: &str) {
    println!("➜ Done: {}", text);
}

pub fn err(text: &str) {
    println!("➜ Fail: {}", text);
}

pub fn warn(text: &str) {
    println!("➜ Warn: {}", text);
}

// 脚本输出
pub fn script_log(text: &str) {
    println!("➜ Script output: {}", text);
}

pub fn script_err(text: &str) {
    println!("➜ Script error: {}", text);
}

// 流输出
// pub fn flow_log(text: &str) {
//     pcs!(Cyan => "➜ "; RBold, Cyan => "Script output: "; Cyan => text);
// }
//
// pub fn flow_err(text: &str) {
//     pcs!(Cyan => "➜ "; RBold, Cyan => "Script Error: "; Red => text);
// }

// // 流程引擎输出
// pub fn flow_log(text: &str){
//     pcs!(Green => "➜ "; RBold, RBlue => "Exec: "; RBlue => text);
// }