use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::{from_str, Value};
use crate::conf::runtime::set_runtime_conf;
use crate::core::common::log::interface::{debug, success};

pub fn load_dll_extension(path: &Path) {
    debug(format!("Load dll extension from {}", path.display()).as_str());

}