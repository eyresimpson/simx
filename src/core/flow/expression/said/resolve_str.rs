use crate::core::common::log::interface::debug;
use crate::core::flow::expression::said::entity::Expression;

pub fn resolve_str(expression: &Expression) {
    debug(format!("{:?}", expression).as_str())
}