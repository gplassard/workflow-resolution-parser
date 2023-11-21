use serde_json::Value;
use crate::expression::PathElement;

pub fn evaluate_path_element(path_element: PathElement, context: Value) -> Result<Value, String> {
    match path_element {
        PathElement::AttributePath { name } => {
            match context.get(name.clone()) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Attribute {} does not exist", name.clone()))
            }
        }
        PathElement::IndexPath { index } => {
            match context.get(index as usize) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Index {} does not exist", index))
            }
        }
    }
}