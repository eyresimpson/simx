use engine_common::entity::ext::Extension;
use libloader::libloading::{Library, Symbol};
use std::path::Path;

// 调用插件的初始化函数
pub fn call_dll_extension_init(extension: Extension) -> Result<(), String> {
    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();

    // dll路径
    let dll_path = Path::new(&function_file).join(extension.entry_lib + ".dll");
    let lib = unsafe { Library::new(dll_path) }.expect("Could not load dll");

    unsafe {
        let init: Symbol<unsafe extern "C" fn()> = lib.get("init".as_bytes()).expect("Could not find init function");
        // 调用函数
        init();
    }
    Ok(())
}
