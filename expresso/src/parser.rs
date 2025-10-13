use pest::Parser;
use pest::error::Error;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprValue<'a> {
    Object(Vec<(String, ExprValue<'a>)>),
    Array(Vec<ExprValue<'a>>),
    String(String),
    Number(f64),
    Boolean(bool),
    Ident(&'a str),
    FnCall(&'a str, Box<ExprValue<'a>>),
    Quote(Box<ExprValue<'a>>),
    Null,
}

pub fn parse_expr(expr: &str) -> Result<ExprValue, Error<Rule>> {
    let parsed = ExprParser::parse(Rule::expr, expr)?.next().unwrap();

    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> ExprValue {
        match pair.as_rule() {
            Rule::object => ExprValue::Object({
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_span()
                            .as_str()
                            .to_string();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect()
            }),
            Rule::array => ExprValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => ExprValue::String(
                pair.into_inner()
                    .next()
                    .unwrap()
                    .as_span()
                    .as_str()
                    .to_string(),
            ),
            Rule::number => ExprValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => ExprValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::ident => ExprValue::Ident(pair.as_str()),
            Rule::fn_call => {
                let mut iter = pair.into_inner();
                let name = iter.next().unwrap();
                let params = iter.next().unwrap();
                ExprValue::FnCall(
                    name.as_str(),
                    match params.as_rule() {
                        Rule::parameters => Box::new(ExprValue::Array(
                            params.into_inner().map(parse_value).collect(),
                        )),
                        Rule::splat => Box::new(parse_value(params.into_inner().next().unwrap())),
                        _ => unreachable!(),
                    },
                )
            }
            Rule::quote => {
                ExprValue::Quote(Box::new(parse_value(pair.into_inner().next().unwrap())))
            }
            Rule::null => ExprValue::Null,
            Rule::expr
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::parameters
            | Rule::splat
            | Rule::WHITESPACE => {
                unreachable!()
            }
        }
    }

    Ok(parse_value(parsed))
}

#[cfg(test)]
mod tests {
    use crate::parser::{ExprValue, parse_expr};

    #[test]
    fn parse_fn_call() {
        assert_eq!(
            parse_expr("test(\"hi\", world, 64.5, false, {\"my\": [value]})").unwrap(),
            ExprValue::FnCall(
                "test",
                Box::new(ExprValue::Array(vec![
                    ExprValue::String("hi".into()),
                    ExprValue::Ident("world"),
                    ExprValue::Number(64.5),
                    ExprValue::Boolean(false),
                    ExprValue::Object(vec![(
                        "my".to_string(),
                        ExprValue::Array(vec![ExprValue::Ident("value")])
                    )])
                ]))
            )
        )
    }
}
