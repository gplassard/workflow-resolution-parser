use crate::expression::Expression;
use serde_json::{json, Value};
use crate::evaluate::evaluate_field_accessor::evaluate_field_accessor;

pub fn evaluate_expression(exp: Expression, context: Value) -> Result<Expression, String> {
    match exp {
        Expression::JsonValue { value } => Ok(Expression::JsonValue { value }),
        Expression::Template { field_accessor } => {
            let evaluated_expression = evaluate_field_accessor(field_accessor.path, context)?;
            Ok(Expression::JsonValue {
                value: evaluated_expression,
            })
        }
        Expression::StringConcatenation { parts } => Ok(Expression::JsonValue {
            value: json!(format!("{{ {:?} }}", parts)),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate::evaluate_expression::evaluate_expression;
    use crate::expression::{Expression, FieldAccessor, PathElement};
    use serde_json::json;

    #[test]
    fn test_evaluate_json_value() {
        let result = evaluate_expression(
            Expression::JsonValue {
                value: json!("hello"),
            },
            json!({}),
        );
        let expected = Ok(Expression::JsonValue {
            value: json!("hello"),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_template_expression() {
        let result = evaluate_expression(
            Expression::Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "foo".to_string(),
                    }]
                },
            },
            json!({"foo": ["bar", "baz"]}),
        );
        let expected = Ok(Expression::JsonValue {
            value: json!(["bar", "baz"]),
        });
        assert_eq!(result, expected);
    }
}
