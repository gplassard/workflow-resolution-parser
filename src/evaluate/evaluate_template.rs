use serde_json::Value;

use crate::evaluate::evaluate_field_accessor::evaluate_field_accessor;
use crate::evaluate::evaluate_function::evaluate_function;
use crate::expression::Template;

pub fn evaluate_template(template: Template, context: Value) -> Result<Value, String> {
    let mut current_value = evaluate_field_accessor(template.field_accessor.path, context)?;
    for function in template.functions {
        current_value = evaluate_function(function, current_value)?;
    }
    Ok(current_value)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::evaluate::evaluate_template::evaluate_template;
    use crate::expression::{FieldAccessor, Function, FunctionName, Template};
    use crate::expression::PathElement::AttributePath;

    #[test]
    fn test_evaluate_template() {
        let result = evaluate_template(
            Template {
                field_accessor: FieldAccessor {
                    path: vec![AttributePath {
                        name: "hello".to_string(),
                    }]
                },
                functions: vec![],
            },
            json!({"hello": "world"}),
        );
        let expected = Ok(json!("world"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_template_with_functions() {
        let result = evaluate_template(
            Template {
                field_accessor: FieldAccessor {
                    path: vec![AttributePath {
                        name: "hello".to_string(),
                    }]
                },
                functions: vec![
                    Function {
                        name: FunctionName::Upper
                    },
                    Function {
                        name: FunctionName::Trim
                    }
                ],
            },
            json!({"hello": "   world   "}),
        );
        let expected = Ok(json!("WORLD"));
        assert_eq!(result, expected);
    }
}
