use crate::evaluate::evaluate_expression::evaluate_expression;
use crate::expression::Expression;
use serde_json::Value;

mod evaluate_expression;
mod evaluate_field_accessor;
mod evaluate_path_element;
mod evaluate_template;
mod evaluate_function;

pub fn evaluate(exp: Expression, context: Value) -> Result<Expression, String> {
    evaluate_expression(exp, context)
}
