use serde_json::json;
use crate::expression::evaluate;
use crate::parse::parse_expression;

mod expression;
mod parse;

fn main() {
    let exp = parse_expression("{{foo.bar[1].baz}}");
    println!("{:?}", exp);
    println!("{:?}", evaluate(exp.unwrap().1, json!({
        "foo": {
            "bar": [
                {},
                {
                    "baz": "foobar"
                },
            ]
        }
    })));
}
