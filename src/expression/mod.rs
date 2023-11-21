use serde_json::Value;

#[derive(PartialEq, Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Expression {
    JsonValue {
        value: Value,
    },
    TemplateExpression {
        expression: TemplateExpression,
    },
    #[allow(dead_code)]
    StringConcatenation {
        parts: Vec<Expression>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub enum TemplateExpression {
    FieldAccessor { path: Vec<PathElement> },
}

#[derive(PartialEq, Debug, Clone)]
pub enum PathElement {
    AttributePath { name: String },
    IndexPath { index: i32 },
}

pub fn evaluate(exp: Expression, context: Value) -> Result<Expression, String> {
    match exp {
        Expression::JsonValue { value } => Ok(Expression::JsonValue { value }),
        Expression::TemplateExpression { expression } => {
            let evaluated_expression = evaluate_template_expression(expression, context)?;
            Ok(Expression::JsonValue {
                value: evaluated_expression
            }),
        },
        Expression::StringConcatenation { parts } => Ok(Expression::JsonValue {
            value: json!(format!("{{ {:?} }}", parts)),
        }),
    }
}

pub fn evaluate_template_expression(exp: TemplateExpression, context: Value) -> Result<Value, String> {
    match exp {
        TemplateExpression::FieldAccessor { path } => {
            evaluate_field_accessor(path, context)
        }
    }
}

pub fn evaluate_field_accessor(path: Vec<PathElement>, context: Value) -> Result<Value, String> {
    let mut current_value = context;
    for path_element in path {
        current_value = evaluate_path_element(path_element, current_value)?;
    }
    Ok(current_value)
}

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
