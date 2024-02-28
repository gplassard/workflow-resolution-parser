use crate::evaluate::evaluate;
use crate::parse::parse_expression;
use serde_json::json;

mod evaluate;
mod expression;
mod parse;

fn main() {
    let exp = parse_expression("{{foo.bar[1].baz | upper | trim}}");
    println!("{:?}", exp);
    println!(
        "{:?}",
        evaluate(
            exp.unwrap().1,
            json!({
                "foo": {
                    "bar": [
                        {},
                        {
                            "baz": "  foobar  "
                        },
                    ]
                }
            })
        )
    );
}
