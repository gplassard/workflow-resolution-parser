use nom::IResult;
use nom::character::complete::anychar;
use nom::multi::many0;
use serde_json::json;
use serde_json::value::Value;

#[derive(PartialEq, Debug, Clone)]
enum ResolvedValue {
    JsonValue {
        value: Value
    },
    ConcatExpression {
        values: Vec<ResolvedValue>
    },
}

fn main() {
    println!("{:?}", parse("Hello world"));
}

fn parse(input: &str) -> IResult<&str, ResolvedValue> {
    let (remaining, parsed) = many0(anychar)(input)?;
    let string: String = parsed.into_iter().collect();
    Ok((
        remaining,
        ResolvedValue::JsonValue { value: json!(string) }
    ))
}

#[cfg(test)]
mod tests {
    use crate::ResolvedValue::JsonValue;

    use super::*;

    #[test]
    fn test_parse_whole_string() {
        let result = parse("Hello world");
        let expected = Ok(("", JsonValue {value: json!("Hello world")}));
        assert_eq!(result, expected);
    }
}
