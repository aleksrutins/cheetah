use std::{collections::HashMap, error::Error, fmt::Display};

use crate::parser::ExprValue;

pub type EvalFn = fn(Vec<EvalResult>) -> EvalResult;

#[derive(Debug, PartialEq, Clone)]
pub struct EvalContext<'a> {
    pub vars: HashMap<String, ExprValue<'a>>,
    pub fns: HashMap<String, EvalFn>,
}

#[derive(Debug, Clone)]
pub struct NoSuchIdentError {
    pub ident: String,
}

impl Error for NoSuchIdentError {}
impl Display for NoSuchIdentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("No such identifier: {}", self.ident))
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum EvalResult<'a> {
    String(String),
    Array(Vec<EvalResult<'a>>),
    Object(Vec<(String, EvalResult<'a>)>),
    Number(f64),
    Boolean(bool),
    Quoted(Box<ExprValue<'a>>, EvalContext<'a>),
    None,
}

impl<'a> Into<ExprValue<'a>> for EvalResult<'a> {
    fn into(self) -> ExprValue<'a> {
        match self {
            EvalResult::String(s) => ExprValue::String(s),
            EvalResult::Array(eval_results) => {
                ExprValue::Array(eval_results.iter().map(|r| r.clone().into()).collect())
            }
            EvalResult::Object(items) => ExprValue::Object(
                items
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone().into()))
                    .collect(),
            ),
            EvalResult::Number(f) => ExprValue::Number(f),
            EvalResult::Boolean(b) => ExprValue::Boolean(b),
            EvalResult::Quoted(expr_value, _) => ExprValue::Quote(expr_value),
            EvalResult::None => ExprValue::Null,
        }
    }
}

impl<'a> Display for EvalResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => f.write_str(&s),
            EvalResult::Array(v) => {
                f.write_str("[")?;
                for item in v {
                    item.fmt(f)?;

                    if item != v.last().unwrap() {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("]")
            }
            EvalResult::Object(items) => {
                f.write_str("{")?;
                for item in items {
                    f.write_fmt(format_args!("{}: {}", item.0, item.1))?;
                }
                f.write_str("}")
            }
            EvalResult::Number(n) => f.write_fmt(format_args!("{}", n)),
            EvalResult::Boolean(b) => f.write_fmt(format_args!("{}", b)),
            EvalResult::Quoted(q, _) => f.write_fmt(format_args!("'{:?}", q)),
            EvalResult::None => f.write_str("null"),
        }
    }
}

pub fn eval<'a>(
    value: ExprValue<'a>,
    ctx: EvalContext<'a>,
) -> Result<EvalResult<'a>, NoSuchIdentError> {
    Ok(match value {
        ExprValue::String(s) => EvalResult::String(s.to_string()),
        ExprValue::Array(a) => EvalResult::Array(
            a.iter()
                .map(|v| eval(v.clone(), ctx.clone()))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        ExprValue::Object(items) => EvalResult::Object(
            items
                .iter()
                .map(|(key, val)| Ok((key.to_string(), eval(val.clone(), ctx.clone())?)))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        ExprValue::Number(n) => EvalResult::Number(n),
        ExprValue::Boolean(b) => EvalResult::Boolean(b),
        ExprValue::Ident(i) => eval(
            (ctx.vars.get(&i.to_string()).ok_or(NoSuchIdentError {
                ident: i.to_string(),
            }))?
            .clone(),
            ctx,
        )?,
        ExprValue::FnCall(name, expr_values) => {
            (ctx.fns.get(&name.to_string()).ok_or(NoSuchIdentError {
                ident: name.to_string(),
            }))?(
                if let EvalResult::Array(args) = eval(expr_values.as_ref().clone(), ctx)? {
                    args
                } else {
                    vec![]
                },
            )
        }
        ExprValue::Quote(q) => EvalResult::Quoted(q, ctx.clone()),
        ExprValue::Null => EvalResult::None,
    })
}

#[cfg(test)]
pub(crate) fn test<'a>(args: Vec<EvalResult>) -> EvalResult {
    if let Some(EvalResult::String(str)) = args.first()
        && let Some(EvalResult::Number(n)) = args.get(1)
    {
        EvalResult::String(format!("{str} {n}"))
    } else {
        EvalResult::None
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::test;
    use std::collections::HashMap;

    use crate::{
        eval::{EvalContext, EvalFn, EvalResult, eval},
        parser::ExprValue,
    };

    #[test]
    fn eval_tree_test() {
        let mut fns = HashMap::new();
        fns.insert("test".to_string(), test as EvalFn);
        let mut vars = HashMap::new();
        vars.insert("hello".to_string(), ExprValue::String("world".into()));
        assert_eq!(
            eval(
                ExprValue::FnCall(
                    "test",
                    Box::new(ExprValue::Array(vec![
                        ExprValue::Ident("hello"),
                        ExprValue::Number(3.14)
                    ]))
                ),
                EvalContext { fns, vars }
            )
            .unwrap(),
            EvalResult::String("world 3.14".to_string())
        );
    }
}
