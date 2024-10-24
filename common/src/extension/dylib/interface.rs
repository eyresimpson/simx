use crate::entity::extension::Extension;
use crate::logger::interface::fail;
use libloading::{Library, Symbol};
use std::path::Path;

pub fn call_dylib_extension_init(extension: Extension) -> Result<(), String> {

    // 取方法所在插件文件名（相对于插件根目录）
    let function_file = extension.path.as_ref().unwrap();
    // dylib路径
    let dylib_path = Path::new(&function_file).join(extension.entry_lib + ".dylib");
    let lib = unsafe { Library::new(dylib_path) }.expect("Could not load dylib");

    unsafe {
        let init: Symbol<unsafe extern "C" fn() -> bool> = lib.get("init".as_bytes()).expect("Could not find init function");
        // 调用函数
        if !init() {
            fail(format!("Call lib {} init failed ", extension.name).as_str())
        }
    }
    Ok(())
}
