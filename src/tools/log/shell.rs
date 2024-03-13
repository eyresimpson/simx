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