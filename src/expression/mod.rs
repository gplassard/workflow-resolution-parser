use serde_json::{json, Value};

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

pub fn evaluate(exp: Expression, context: Value) -> Expression {
    match exp {
        Expression::JsonValue { value } => Expression::JsonValue { value },
        Expression::TemplateExpression { expression } => Expression::JsonValue {
            value: evaluate_template_expression(expression, context),
        },
        Expression::StringConcatenation { parts } => Expression::JsonValue {
            value: json!(format!("{{ {:?} }}", parts)),
        },
    }
}

pub fn evaluate_template_expression(exp: TemplateExpression, context: Value) -> Value {
    match exp {
        TemplateExpression::FieldAccessor { path } => {
            evaluate_field_accessor(path, context)
        }
    }
}

pub fn evaluate_field_accessor(path: Vec<PathElement>, context: Value) -> Value {
    let mut current_value = context;
    for path_element in path {
        current_value = evaluate_path_element(path_element, current_value);
    }
    current_value
}

pub fn evaluate_path_element(path_element: PathElement, context: Value) -> Value {
    match path_element {
        PathElement::AttributePath { name } => {
            context.get(name).unwrap().clone()
        }
        PathElement::IndexPath { index } => {
            context.get(index as usize).unwrap().clone()
        }
    }
}
