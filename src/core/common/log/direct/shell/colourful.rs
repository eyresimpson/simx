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
    pcs!(Purple => text);
}
// 面前临时将两者合二为一，不再关注其区别，都以Purple显示
pub fn script_err(text: &str) {
    pcs!(Purple => text);
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