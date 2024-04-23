use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

// 表达式对象
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Expression {
    // 表达式字符串
    pub(crate) expression_str: String,
    // 表达式类型
    pub(crate) expression_type: ExpressionType,
    // 表达式结果
    pub(crate) expression_result: HashMap<String, ()>,
}

// 表达式类型
// 目前分为字符串、数值（包括浮点），布尔，操作（方法）
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ExpressionType {
    // 字符串表达式
    STR,
    // 数值表达式
    NUM,
    // 布尔表达式
    BOOL,
    // 操作表达式
    OPT,
}

// 表达式处理器
// 目前仅支持Said表达式处理器
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum ExpressionHandle {
//     // Said表达式
//     SAID
// }