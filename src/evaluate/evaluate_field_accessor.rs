use crate::expression::PathElement;
use serde_json::Value;
use crate::evaluate::evaluate_path_element::evaluate_path_element;

pub fn evaluate_field_accessor(path: Vec<PathElement>, context: Value) -> Result<Value, String> {
    let mut current_value = context;
    for path_element in path {
        current_value = evaluate_path_element(path_element, current_value)?;
    }
    Ok(current_value)
}

#[cfg(test)]
mod tests {
    use crate::evaluate::evaluate_field_accessor::evaluate_field_accessor;
    use crate::expression::PathElement;
    use serde_json::json;

    #[test]
    fn test_evaluate_single_element_field_accessor() {
        let result = evaluate_field_accessor(
            vec![PathElement::AttributePath {
                name: "foo".to_string(),
            }],
            json!({"foo": "bar"}),
        );
        let expected = Ok(json!("bar"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_nested_element_field_accessor() {
        let result = evaluate_field_accessor(
            vec![
                PathElement::AttributePath {
                    name: "foo".to_string(),
                },
                PathElement::IndexPath { index: 1 },
            ],
            json!({"foo": ["bar", "baz"]}),
        );
        let expected = Ok(json!("baz"));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_evaluate_empty_field_accessor() {
        let result = evaluate_field_accessor(vec![], json!({"foo": ["bar", "baz"]}));
        let expected = Ok(json!({"foo": ["bar", "baz"]}));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_non_existent_field_accessor() {
        let result = evaluate_field_accessor(
            vec![PathElement::IndexPath { index: 999 }],
            json!({"foo": ["bar", "baz"]}),
        );
        let expected = Err("Index 999 does not exist".to_string());
        assert_eq!(result, expected);
    }
}
