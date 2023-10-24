use crate::expression::evaluate;
use crate::parse::parse_expression;

mod expression;
mod parse;

fn main() {
    let exp = parse_expression("{{trUe}}");
    println!("{:?}", exp);
    println!("{:?}", evaluate(exp.unwrap().1));
}
