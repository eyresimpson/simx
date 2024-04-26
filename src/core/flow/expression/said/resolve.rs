use crate::core::common::log::interface::debug;
use crate::core::flow::expression::said::entity::{Expression, ExpressionType};
use crate::core::flow::expression::said::resolve_bool::resolve_bool;
use crate::core::flow::expression::said::resolve_num::resolve_num;
use crate::core::flow::expression::said::resolve_opt::resolve_opt;
use crate::core::flow::expression::said::resolve_str::resolve_str;

pub fn resolve(expression: Box<Expression>) -> Box<Expression> {
    // 处理表达式
    match expression.expression_type {
        ExpressionType::STR => { resolve_str(&expression) }
        ExpressionType::NUM => { resolve_num(&expression) }
        ExpressionType::BOOL => { resolve_bool(&expression) }
        ExpressionType::OPT => { resolve_opt(&expression) }
    }

    debug(format!("{:?}", expression).as_str());

    return expression;
}
