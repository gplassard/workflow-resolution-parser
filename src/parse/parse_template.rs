use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;

use crate::expression::Expression;
use crate::parse::parse_field_accessor::parse_field_accessor;

pub fn parse_template(input: &str) -> IResult<&str, Expression> {
    let (remaining, parsed) = delimited(
        tag("{{"),
        preceded(space0, terminated(parse_field_accessor, space0)),
        tag("}}"),
    )(input)?;

    Ok((
        remaining,
        Expression::Template { field_accessor: parsed },
    ))
}

#[cfg(test)]
mod tests {
    use crate::expression::{Expression, PathElement, FieldAccessor};
    use crate::parse::parse_template::parse_template;

    #[test]
    fn test_parse() {
        let result = parse_template("{{true}}");
        let expected = Ok((
            "",
            Expression::Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "true".to_string(),
                    }],
                },
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_spaces() {
        let result = parse_template("{{ false }}");
        let expected = Ok((
            "",
            Expression::Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "false".to_string(),
                    }],
                },
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_remain() {
        let result = parse_template("{{TrUe}}WithOtherStuff");
        let expected = Ok((
            "WithOtherStuff",
            Expression::Template {
                field_accessor: FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "TrUe".to_string(),
                    }],
                },
            },
        ));
        assert_eq!(result, expected);
    }
}
