use serde_json::Value;
use crate::evaluate::evaluate_field_accessor::evaluate_field_accessor;
use crate::expression::TemplateExpression;

pub fn evaluate_template_expression(exp: TemplateExpression, context: Value) -> Result<Value, String> {
    match exp {
        TemplateExpression::FieldAccessor { path } => {
            evaluate_field_accessor(path, context)
        }
    }
}
