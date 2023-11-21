use serde_json::{json, Value};
use crate::evaluate::evaluate_template_expression::evaluate_template_expression;
use crate::expression::Expression;

pub fn evaluate_expression(exp: Expression, context: Value) -> Result<Expression, String> {
    match exp {
        Expression::JsonValue { value } => Ok(Expression::JsonValue {
            value
        }),
        Expression::TemplateExpression { expression } => {
            let evaluated_expression = evaluate_template_expression(expression, context)?;
            Ok(Expression::JsonValue {
                value: evaluated_expression
            })
        },
        Expression::StringConcatenation { parts } => Ok(Expression::JsonValue {
            value: json!(format!("{{ {:?} }}", parts))
        }),
    }
}