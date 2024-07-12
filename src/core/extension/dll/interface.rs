use std::path::Path;

use crate::core::common::log::interface::debug;

pub fn load_dll_extension(path: &Path) {
    debug(format!("Load dll extension from {}", path.display()).as_str());

}