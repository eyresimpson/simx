use serde_derive::{Deserialize, Serialize};

// 获取语言
// pub fn get_language_str(tag: String) -> String {
//     // let language = Language::Chinese;
//     // println!("{}", tag);
//     // match language {
//     //     Language::Chinese => println!("Chinese"),
//     //     Language::English => println!("English"),
//     // }
//     "sss".to_string()
// }

// 加载多语言包到内存中（默认仅加载配置中指定的）
pub fn load_language_package() {
    // match get_simx_config().engine.language {
    //     Language::Chinese => println!("Chinese"),
    //     Language::English => println!("English"),
    // }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum Language {
    Chinese,
    #[default]
    English,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LanguagePackage {
    // 引擎初始化提示
    pub engine_init_info: String,
    // 引擎banner
    pub engine_banner: String,
    // 引擎运行开始提示
    pub engine_run_start_info: String,
    // 引擎运行结束提示
    pub engine_run_end_info: String,
    // 找不到路径
    pub cannot_find_path: String,
    // 找不到文件
    pub cannot_find_file: String,
    // 找不到插件
    pub cannot_find_extension: String,
    // 找不到流文件
    pub cannot_find_flow_file: String,
    // 找不到脚本文件
    pub cannot_find_script_file: String,
    // 调试：流节点开始运行信息
    pub debug_flow_node_start: String,
    // 调试：流节点结束运行信息
    pub debug_flow_node_end: String,
}
