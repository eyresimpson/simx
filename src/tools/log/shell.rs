use color_string::Font::*;
use color_string::{cs, fonts, pcs, wcs, wf, Colored, FontTool};

pub fn info(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Blue => "Info: "; Blue => text);
}

pub fn success(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Green => "Success: "; Green => text);
}

pub fn err(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Red => "Success: "; Red => text);
}

pub fn warn(text: &str) {
    pcs!(Cyan => "➜ "; RBold, Yellow => "Success: "; Yellow => text);
}