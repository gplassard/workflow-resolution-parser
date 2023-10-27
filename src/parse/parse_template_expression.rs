use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::opt;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::terminated;

use crate::expression::TemplateExpression;
use crate::parse::parse_path_element::{parse_attribute_path, parse_index_path};

pub fn parse_template_expression(input: &str) -> IResult<&str, TemplateExpression> {
    let (remaining, parsed) = many0(terminated(alt((parse_attribute_path, parse_index_path)), opt(char('.'))))(input)?;

    Ok((
        remaining,
        TemplateExpression::FieldAccessor {
            path: parsed
        }
    ))
}


#[cfg(test)]
mod tests {
    use crate::expression::TemplateExpression;
    use crate::expression::PathElement::{AttributePath, IndexPath};
    use crate::parse::parse_template_expression::parse_template_expression;

    #[test]
    fn test_parse_single_attribute_path() {
        let result = parse_template_expression("foo");
        let expected = Ok(("", TemplateExpression::FieldAccessor {
            path: vec!(
                AttributePath { name: "foo".to_string() }
            )
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_multiple_attribute_path() {
        let result = parse_template_expression("foo.bar");
        let expected = Ok(("", TemplateExpression::FieldAccessor {
            path: vec!(
                AttributePath { name: "foo".to_string() },
                AttributePath { name: "bar".to_string() },
            )
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_index_path() {
        let result = parse_template_expression("foo[0]");
        let expected = Ok(("", TemplateExpression::FieldAccessor {
            path: vec!(
                AttributePath { name: "foo".to_string() },
                IndexPath { index: 0 },
            )
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_nested() {
        let result = parse_template_expression("foo[0].bar[1]");
        let expected = Ok(("", TemplateExpression::FieldAccessor {
            path: vec!(
                AttributePath { name: "foo".to_string() },
                IndexPath { index: 0 },
                AttributePath { name: "bar".to_string() },
                IndexPath { index: 1 },
            )
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_remains() {
        let result = parse_template_expression("foo[0].bar[1] andotherstuff");
        let expected = Ok((" andotherstuff", TemplateExpression::FieldAccessor {
            path: vec!(
                AttributePath { name: "foo".to_string() },
                IndexPath { index: 0 },
                AttributePath { name: "bar".to_string() },
                IndexPath { index: 1 },
            )
        }));
        assert_eq!(result, expected);
    }

}
