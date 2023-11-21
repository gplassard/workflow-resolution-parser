use serde_json::Value;
use crate::evaluate::evaluate_expression::evaluate_expression;
use crate::expression::Expression;

mod evaluate_expression;
mod evaluate_template_expression;
mod evaluate_field_accessor;
mod evaluate_path_element;

pub fn evaluate(exp: Expression, context: Value) -> Result<Expression, String> {
    evaluate_expression(exp, context)
}