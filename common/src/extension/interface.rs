use crate::entity::extension::Extension;

// 注意，此方法里，都是遵循Simx 插件规范的情况

// 加载扩展
pub fn load_extension(extension: Extension) {}

// 卸载扩展
pub fn unload_extension(extension: Extension) {}

// 检查扩展
pub fn check_extension(extension: Extension) {}

// 调用rust编写的扩展（直接是结构体）
pub fn invoke_extension_func_common(extension: Extension) {}

// 调用非rust编写的扩展（通过二进制或Json字符串）
pub fn invoke_extension_func_natural() {}

// 调用脚本接口
pub fn invoke_extension_func_script() {}

// 调用restful接口
pub fn invoke_extension_func_restful() {}

// 调用socket接口
pub fn invoke_extension_func_socket() {}