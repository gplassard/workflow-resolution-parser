use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;

use crate::expression::Expression;
use crate::parse::parse_template_expression::parse_template_expression;

pub fn parse_template(input: &str) -> IResult<&str, Expression> {
    let (remaining, parsed) = delimited(
        tag("{{"),
        preceded(space0, terminated(parse_template_expression, space0)),
        tag("}}"),
    )(input)?;

    Ok((
        remaining,
        Expression::TemplateExpression { expression: parsed },
    ))
}

#[cfg(test)]
mod tests {
    use crate::expression::{Expression, PathElement, TemplateExpression};
    use crate::parse::parse_template::parse_template;

    #[test]
    fn test_parse() {
        let result = parse_template("{{true}}");
        let expected = Ok((
            "",
            Expression::TemplateExpression {
                expression: TemplateExpression::FieldAccessor {
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
            Expression::TemplateExpression {
                expression: TemplateExpression::FieldAccessor {
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
            Expression::TemplateExpression {
                expression: TemplateExpression::FieldAccessor {
                    path: vec![PathElement::AttributePath {
                        name: "TrUe".to_string(),
                    }],
                },
            },
        ));
        assert_eq!(result, expected);
    }
}
