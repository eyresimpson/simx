use std::path::Path;

use crate::core::common::log::interface::warn;

pub fn exec_sql_script(path: &Path){
    warn(path.to_str().unwrap())
}
