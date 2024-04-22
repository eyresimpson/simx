use crate::core::flow::expression::said::entity::{Expression, ExpressionType};
use crate::core::flow::expression::said::resolve::resolve;

// 表达式通用出口，所有对表达式的调用都应从此处开始
// 目前仅支持解析 Said 表达式
pub fn resolve_said_expression(expression_str: &str, expression_type: &str) -> Box<Expression> {
    let mut expression = Expression {
        expression_str: expression_str.to_string(),
        expression_type: ExpressionType::STR,
        expression_result: (),
    };
    match expression_type {
        "str" => { expression.expression_type = ExpressionType::STR }
        "num" => { expression.expression_type = ExpressionType::NUM }
        "bool" => { expression.expression_type = ExpressionType::BOOL }
        "opt" => { expression.expression_type = ExpressionType::OPT }
        _ => { expression.expression_type = ExpressionType::STR }
    }
    // 目前仅支持said表达式
    resolve(Box::new(expression))
}