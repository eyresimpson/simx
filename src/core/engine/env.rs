use crate::tools::log::shell::{info, success};
use crate::tools::sys::common::get_system_version;

// 环境检查
pub fn check() {
    info("Check Workspace...");
    // 检查操作系统版本（支持Windows 10+/Macos 14+/Linux 部分发行版），这个检查不会引起系统退出
    // 如果操作系统版本检查不通过，会在控制台显示一个警告
    get_system_version();

    success("Check Workspace Done.");
    
}

// 加载配置文件
pub fn load_configuration() {
    // 优先级：运行目录配置 > 其他配置
    info("Engine Loading Configuration...");
    // 加载配置
    success("Engine Loading Configuration Done.");

}