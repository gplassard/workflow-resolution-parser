use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

use crate::expression::{Function, FunctionName};

pub fn parse_function(input: &str) -> IResult<&str, Function> {
    let (remaining, parsed) = alt((
        value(
            FunctionName::Trim,
            tag(FunctionName::Trim.to_string().to_lowercase().as_str()),
        ),
        value(
            FunctionName::Upper,
            tag(FunctionName::Upper.to_string().to_lowercase().as_str()),
        ),
        value(
            FunctionName::Lower,
            tag(FunctionName::Lower.to_string().to_lowercase().as_str()),
        ),
        value(
            FunctionName::Length,
            tag(FunctionName::Length.to_string().to_lowercase().as_str()),
        ),
    ))(input)?;

    Ok((remaining, Function { name: parsed }))
}

#[cfg(test)]
mod tests {
    use crate::expression::{Function, FunctionName};
    use crate::parse::parse_function::parse_function;

    #[test]
    fn test_parse_trim() {
        let result = parse_function("trim");
        let expected = Ok((
            "",
            Function {
                name: FunctionName::Trim,
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_upper() {
        let result = parse_function("upper");
        let expected = Ok((
            "",
            Function {
                name: FunctionName::Upper,
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_lower() {
        let result = parse_function("lower");
        let expected = Ok((
            "",
            Function {
                name: FunctionName::Lower,
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_length() {
        let result = parse_function("length");
        let expected = Ok((
            "",
            Function {
                name: FunctionName::Length,
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_length_remaining() {
        let result = parse_function("lengthAndMore");
        let expected = Ok((
            "AndMore",
            Function {
                name: FunctionName::Length,
            },
        ));
        assert_eq!(result, expected);
    }
}
