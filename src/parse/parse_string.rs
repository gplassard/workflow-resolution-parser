use nom::IResult;
use serde_json::json;

use crate::expression::Expression;

pub fn parse_string_as_expression(input: &str) -> IResult<&str, Expression> {
    Ok((
        "",
        Expression::JsonValue {
            value: json!(input)
        }
    ))
}

pub fn parse_string(input: &str) -> IResult<&str, &str> {
    match input.find("}}") {
        Some(index) => Ok((&input[index..], &input[..index])),
        None => Ok(("", input))
    }
}


#[cfg(test)]
mod tests {
    use crate::parse::parse_string::parse_string;

    #[test]
    fn test_parse() {
        let result = parse_string("true");
        let expected = Ok(("", "true"));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_empty() {
        let result = parse_string("");
        let expected = Ok(("", ""));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_stops_at_closing_brackets() {
        let result = parse_string("aaaaa }} after");
        let expected = Ok(("}} after", "aaaaa "));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_doesnt_stops_at_single_bracket() {
        let result = parse_string("aaaaa } after");
        let expected = Ok(("", "aaaaa } after"));
        assert_eq!(result, expected);
    }
}
