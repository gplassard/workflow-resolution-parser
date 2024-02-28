use std::any::Any;

use serde_json::{json, Value};

use crate::expression::{Function, FunctionName};

pub fn evaluate_function(function: Function, context: Value) -> Result<Value, String> {
    context.as_str()
        .map(|context| match function.name {
            FunctionName::LENGTH => {
                json!(context.len())
            }
            FunctionName::UPPER => {
                json!(context.to_uppercase())
            }
            FunctionName::LOWER => {
                json!(context.to_lowercase())
            }
            FunctionName::TRIM => {
                json!(context.trim())
            }
        })
        .ok_or(format!("Expected value to be a string, got {:?}", context.type_id()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::evaluate::evaluate_function::evaluate_function;
    use crate::expression::{Function, FunctionName};

    #[test]
    fn test_evaluate_length() {
        let result = evaluate_function(
            Function {
                name: FunctionName::LENGTH,
            },
            json!("helloworld"),
        );
        let expected = Ok(json!(10));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_evaluate_upper() {
        let result = evaluate_function(
            Function {
                name: FunctionName::UPPER,
            },
            json!("helloworld"),
        );
        let expected = Ok(json!("HELLOWORLD"));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_evaluate_lower() {
        let result = evaluate_function(
            Function {
                name: FunctionName::LOWER,
            },
            json!("HELLOWORLD"),
        );
        let expected = Ok(json!("helloworld"));
        assert_eq!(result, expected);
    }
    #[test]
    fn test_evaluate_trim() {
        let result = evaluate_function(
            Function {
                name: FunctionName::TRIM,
            },
            json!("  abc "),
        );
        let expected = Ok(json!("abc"));
        assert_eq!(result, expected);
    }
}
