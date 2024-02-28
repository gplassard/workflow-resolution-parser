use std::fmt;
use serde_json::Value;

#[derive(PartialEq, Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Expression {
    JsonValue {
        value: Value,
    },
    Template {
        template: Template,
    },
    #[allow(dead_code)]
    StringConcatenation {
        parts: Vec<Expression>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Template {
    pub field_accessor: FieldAccessor,
    pub functions: Vec<Function>
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub name: FunctionName,
}

#[derive(PartialEq, Debug, Clone)]
pub enum FunctionName {
    LENGTH,
    UPPER,
    LOWER,
    TRIM
}

impl fmt::Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FieldAccessor {
    pub path: Vec<PathElement>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PathElement {
    AttributePath { name: String },
    IndexPath { index: i32 },
}
