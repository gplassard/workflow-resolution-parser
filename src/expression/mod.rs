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
