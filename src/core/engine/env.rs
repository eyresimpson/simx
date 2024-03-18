use crate::tools::log::shell::{info, success};
use crate::tools::sys::common::get_system_version;

// 环境检查
pub fn check() -> bool {
    info("Check Workspace...");
    // 检查操作系统版本（支持Windows 10+/Macos 14+/Linux 部分发行版），这个检查不会引起系统退出
    // 如果操作系统版本检查不通过，会在控制台显示一个警告
    get_system_version();

    // 检查python版本
    // 首先检查配置文件中是否有具体配置，如果有则直接使用配置文件中的内容，如果没有再去找系统默认的配置
    check_python_env();

    success("Check Workspace Done.");
    return true;
}

pub fn check_python_env() {}