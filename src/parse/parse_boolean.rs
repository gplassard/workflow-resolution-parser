use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::value;
use nom::IResult;
use serde_json::json;

use crate::expression::Expression;

pub fn parse_boolean(input: &str) -> IResult<&str, Expression> {
    alt((
        value(
            Expression::JsonValue { value: json!(true) },
            tag_no_case("true"),
        ),
        value(
            Expression::JsonValue { value: json!(false)},
            tag_no_case("false"),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::expression::Expression::JsonValue;
    use crate::parse::parse_boolean::parse_boolean;

    #[test]
    fn test_parse_true() {
        let result = parse_boolean("true");
        let expected = Ok(("", JsonValue { value: json!(true) }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_false() {
        let result = parse_boolean("false");
        let expected = Ok((
            "",
            JsonValue {
                value: json!(false),
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_case() {
        let result = parse_boolean("TrUe");
        let expected = Ok(("", JsonValue { value: json!(true) }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_remain() {
        let result = parse_boolean("TrUeAndOtherStuff");
        let expected = Ok(("AndOtherStuff", JsonValue { value: json!(true) }));
        assert_eq!(result, expected);
    }
}
