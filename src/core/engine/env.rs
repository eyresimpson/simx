use crate::tools::log::shell::{info, success};

// 环境检查
pub fn check() -> bool {
    info("Check Workspace...");
    // 检查运行目录下是否有配置文件夹
    
    // 检查运行目录下是否有数据库目录
    
    success("Check Workspace Done.");
    return true;
}