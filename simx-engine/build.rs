extern crate winres;
fn main() {
    // 仅windows下有效
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        // 设置图标
        res.set_icon("resource/logo.ico")
            // 设置内部名称
            .set("InternalName", "Simx")
            // 设置
            .set("FileDescription", "Simx Engine")
            // 设置产品名称
            .set("ProductName", "Simx")
            // 设置版权信息
            .set("LegalCopyright", "Copyright © 2024 NJ Labs")
            // 设置公司名称
            .set("CompanyName", "Noah Jones")
            // 设置产品版本
            .set("ProductVersion", "0.1.1")
            // 设置文件版本
            .set("FileVersion", "0.1.1");
        // 编译资源
        res.compile().expect("Failed to compile resources");
    }
}
