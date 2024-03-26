
use color_string::Font::*;
use color_string::pcs;

pub fn info(text: &str) {
    pcs!(Blue => "➜ "; Blue => text);
}

pub fn success(text: &str) {
    pcs!(Green => "➜ "; Green => text);
}

pub fn err(text: &str) {
    pcs!(Red => "➜ "; Red => text);
}

pub fn warn(text: &str) {
    pcs!(Yellow => "➜ "; Yellow => text);
}

// 脚本输出
pub fn script_log(text: &str) {
    pcs!(Purple => "➜ "; Purple => text);
}

pub fn script_err(text: &str) {
    pcs!(Purple => "➜ "; Red => text);
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