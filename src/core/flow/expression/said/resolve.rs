use crate::core::flow::expression::said::entity::{Expression, ExpressionType};
use crate::core::flow::expression::said::resolve_bool::resolve_bool;
use crate::core::flow::expression::said::resolve_num::resolve_num;
use crate::core::flow::expression::said::resolve_opt::resolve_opt;
use crate::core::flow::expression::said::resolve_str::resolve_str;

pub fn resolve(mut expression: Box<Expression>) -> Box<Expression> {
    // 处理表达式
    match expression.expression_type {
        // ExpressionType::STR => { resolve_str(&expression) }
        ExpressionType::NUM => { resolve_num() }
        ExpressionType::BOOL => { resolve_bool() }
        ExpressionType::OPT => { resolve_opt() }
        _ => {}
    }

    println!("{:?}", expression);

    return expression;
}
