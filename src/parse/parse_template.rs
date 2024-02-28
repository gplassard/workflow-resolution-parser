use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, terminated, tuple};

use crate::expression::Template;
use crate::parse::parse_field_accessor::parse_field_accessor;
use crate::parse::parse_function::parse_function;

pub fn parse_template(input: &str) -> IResult<&str, Template> {
    let functions_parser = separated_list1(pipe_parser, parse_function);

    let (remaining, parsed) = delimited(
        terminated(tag("{{"), space0),
        tuple((parse_field_accessor, opt(preceded(pipe_parser, functions_parser)))),
        preceded(space0, tag("}}")),
    )(input)?;

    let (field_accessor, functions) = parsed;

    Ok((
        remaining,
        Template { field_accessor, functions: functions.unwrap_or_default() },
    ))
}

fn pipe_parser(input: &str) -> IResult<&str, ()> {
    let (remaining, _) = preceded(space0, terminated(tag("|"), space0))(input)?;
    Ok((remaining, ()))
}

#[cfg(test)]
mod tests {
    use crate::expression::{FieldAccessor, Function, FunctionName, PathElement, Template};
    use crate::parse::parse_template::parse_template;

    #[test]
    fn test_parse() {
        let result = parse_template("{{true}}");
        let expected = Ok((
            "",
            Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "true".to_string(),
                    }],
                },
                functions: vec![],
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_spaces() {
        let result = parse_template("{{ false }}");
        let expected = Ok((
            "",
            Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "false".to_string(),
                    }],
                },
                functions: vec![],
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_remain() {
        let result = parse_template("{{TrUe}}WithOtherStuff");
        let expected = Ok((
            "WithOtherStuff",
            Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "TrUe".to_string(),
                    }],
                },
                functions: vec![],
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_with_functions() {
        let result = parse_template("{{ TrUe | lower | trim | upper }}");
        let expected = Ok((
            "",
            Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "TrUe".to_string(),
                    }],
                },
                functions: vec![
                    Function {
                        name: FunctionName::LOWER,
                    },
                    Function {
                        name: FunctionName::TRIM,
                    },
                    Function {
                        name: FunctionName::UPPER,
                    },
                ],
            },
        ));
        assert_eq!(result, expected);
    }
}
