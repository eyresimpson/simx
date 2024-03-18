use color_string::Font::*;
use color_string::pcs;

pub fn info(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Blue => "Info: "; Blue => text);
}

pub fn success(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Green => "Done: "; Green => text);
}

pub fn err(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Red => "Fail: "; Red => text);
}

pub fn warn(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Yellow => "Warn: "; Yellow => text);
}

// 脚本输出
pub fn script_log(text: &str) {
    pcs!(Purple => "➜ "; RBold, Purple => "Script output: "; Purple => text);
}

pub fn script_err(text: &str) {
    pcs!(Purple => "➜ "; RBold, Purple => "Script Error: "; Red => text);
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