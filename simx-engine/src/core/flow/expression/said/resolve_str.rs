use crate::core::flow::expression::said::entity::Expression;
use crate::tools::log::interface::debug;

pub fn resolve_str(expression: &Expression) {
    debug(format!("{:?}", expression).as_str())
}