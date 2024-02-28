use serde_json::Value;

use crate::evaluate::evaluate_field_accessor::evaluate_field_accessor;
use crate::expression::Template;

pub fn evaluate_template(template: Template, context: Value) -> Result<Value, String> {
    evaluate_field_accessor(template.field_accessor.path, context)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::evaluate::evaluate_template::evaluate_template;
    use crate::expression::{FieldAccessor, Template};
    use crate::expression::PathElement::AttributePath;

    #[test]
    fn test_evaluate_template() {
        let result = evaluate_template(
            Template {
                field_accessor: FieldAccessor {
                    path: vec![AttributePath {
                        name: "hello".to_string(),
                    }]
                }
            },
            json!({"hello": "world"}),
        );
        let expected = Ok(json!("world"));
        assert_eq!(result, expected);
    }
}
