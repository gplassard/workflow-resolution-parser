use nom::branch::alt;
use nom::IResult;

use crate::expression::Expression;
use crate::parse::parse_boolean::parse_boolean;
use crate::parse::parse_string::parse_string_as_expression;
use crate::parse::parse_template::parse_template;

mod parse_boolean;
mod parse_path_element;
mod parse_string;
mod parse_template;
mod parse_field_accessor;

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((parse_template, parse_boolean, parse_string_as_expression))(input)
}
