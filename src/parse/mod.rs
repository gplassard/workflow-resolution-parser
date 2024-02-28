use nom::branch::alt;
use nom::IResult;

use crate::expression::Expression;
use crate::parse::parse_boolean::parse_boolean;
use crate::parse::parse_string::parse_string_as_expression;
use crate::parse::parse_template::parse_template;

mod parse_boolean;
mod parse_field_accessor;
mod parse_function;
mod parse_path_element;
mod parse_string;
mod parse_template;

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_template_as_expression,
        parse_boolean,
        parse_string_as_expression,
    ))(input)
}

fn parse_template_as_expression(input: &str) -> IResult<&str, Expression> {
    let (remaining, parsed) = parse_template(input)?;
    Ok((remaining, Expression::Template { template: parsed }))
}
