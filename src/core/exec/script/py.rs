use std::path::Path;

use crate::tools::log::shell::info;

pub fn exec_python_script(path: &Path) {
    let file_name = path.file_name().unwrap().to_str().unwrap_or_default();
    info(file_name);
    // 检查python环境（使用python还是python3）

}
