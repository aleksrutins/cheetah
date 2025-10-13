use pest::Parser;
use pest::error::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

#[derive(Debug, PartialEq)]
pub enum ExprValue<'a> {
    Object(Vec<(&'a str, ExprValue<'a>)>),
    Array(Vec<ExprValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Ident(&'a str),
    FnCall(&'a str, Vec<ExprValue<'a>>),
    Null,
}

pub fn parse_expr(expr: &str) -> Result<ExprValue, Error<Rule>> {
    let parsed = ExprParser::parse(Rule::expr, expr)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> ExprValue {
        match pair.as_rule() {
            Rule::object => ExprValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => ExprValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => ExprValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => ExprValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => ExprValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::ident => ExprValue::Ident(pair.as_str()),
            Rule::fn_call => {
                let mut iter = pair.into_inner();
                ExprValue::FnCall(
                    iter.next().unwrap().as_str(),
                    iter.next().unwrap().into_inner().map(parse_value).collect(),
                )
            }
            Rule::null => ExprValue::Null,
            Rule::expr
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::parameters
            | Rule::WHITESPACE => {
                println!("{:?}", pair.as_rule());
                unreachable!()
            }
        }
    }

    Ok(parse_value(parsed))
}

mod tests {
    use crate::parser::{ExprValue, parse_expr};

    #[test]
    fn parse_fn_call() {
        assert_eq!(
            parse_expr("test(\"hi\", world, 64.5, false, {\"my\": [value]})").unwrap(),
            ExprValue::FnCall(
                "test",
                vec![
                    ExprValue::String("hi"),
                    ExprValue::Ident("world"),
                    ExprValue::Number(64.5),
                    ExprValue::Boolean(false),
                    ExprValue::Object(vec![(
                        "my",
                        ExprValue::Array(vec![ExprValue::Ident("value")])
                    )])
                ]
            )
        )
    }
}
