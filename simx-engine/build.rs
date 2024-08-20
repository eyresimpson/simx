extern crate winres;
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resource/simx.ico") // 如果有图标，可以设置应用图标
            .set("FileDescription", "Simx flow engine core")
            .set("ProductName", "simx")
            .set("LegalCopyright", "Copyright © 2024 NJ Labs")
            .set("CompanyName", "Noah Jones")
            .set("ProductVersion", "1.0.0")
            .set("FileVersion", "1.0.0.0");

        res.compile().expect("Failed to compile resources");
    }
}
