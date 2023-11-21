use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::error::{Error, ErrorKind};
use nom::sequence::delimited;
use nom::IResult;

use crate::expression::PathElement;

pub fn parse_attribute_path(input: &str) -> IResult<&str, PathElement> {
    let (remaining, parsed) = alphanumeric1(input)?;

    Ok((
        remaining,
        PathElement::AttributePath {
            name: parsed.to_string(),
        },
    ))
}

pub fn parse_index_path(input: &str) -> IResult<&str, PathElement> {
    let (remaining, parsed) = delimited(char('['), digit1, char(']'))(input)?;

    let parsed_as_i32: i32 = parsed
        .parse()
        .map_err(|_| nom::Err::Error(Error::new(remaining, ErrorKind::Digit)))?;

    Ok((
        remaining,
        PathElement::IndexPath {
            index: parsed_as_i32,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::expression::PathElement;
    use crate::parse::parse_path_element::{parse_attribute_path, parse_index_path};

    #[test]
    fn test_parse_attribute_path() {
        let result = parse_attribute_path("foo");
        let expected = Ok((
            "",
            PathElement::AttributePath {
                name: "foo".to_string(),
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_attribute_path_stops_at_dot() {
        let result = parse_attribute_path("foo.bar");
        let expected = Ok((
            ".bar",
            PathElement::AttributePath {
                name: "foo".to_string(),
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_index_path() {
        let result = parse_index_path("[123]");
        let expected = Ok(("", PathElement::IndexPath { index: 123 }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_index_path_with_remains() {
        let result = parse_index_path("[123]aaa");
        let expected = Ok(("aaa", PathElement::IndexPath { index: 123 }));
        assert_eq!(result, expected);
    }
}
