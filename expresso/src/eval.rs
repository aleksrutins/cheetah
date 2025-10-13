use std::{any::Any, collections::HashMap, error::Error, ffi::NulError, fmt::Display};

use crate::parser::ExprValue;

pub type EvalFn = fn(Vec<EvalResult>) -> EvalResult;
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

#[derive(PartialEq, Debug)]
pub enum EvalResult {
    String(String),
    Array(Vec<EvalResult>),
    Object(Vec<(String, EvalResult)>),
    Number(f64),
    Boolean(bool),
    None,
}

pub fn eval<'a>(
    value: ExprValue<'a>,
    ctx: &'a EvalContext<'a>,
) -> Result<EvalResult, NoSuchIdentError> {
    Ok(match value {
        ExprValue::String(s) => EvalResult::String(s.to_string()),
        ExprValue::Array(a) => EvalResult::Array(
            a.iter()
                .map(|v| eval(v.clone(), ctx))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        ExprValue::Object(items) => EvalResult::Object(
            items
                .iter()
                .map(|(key, val)| Ok((key.to_string(), eval(val.clone(), ctx)?)))
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
                expr_values
                    .into_iter()
                    .map(|v| eval(v, ctx))
                    .collect::<Result<Vec<_>, _>>()?,
            )
        }
        ExprValue::Null => EvalResult::None,
    })
}

pub(crate) fn test<'a>(args: Vec<EvalResult>) -> EvalResult {
    if let Some(EvalResult::String(str)) = args.get(0)
        && let Some(EvalResult::Number(n)) = args.get(1)
    {
        EvalResult::String(format!("{} {}", str, n))
    } else {
        EvalResult::None
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::test;
    use std::{any::Any, collections::HashMap, error::Error};

    use crate::{
        eval::{EvalContext, EvalFn, EvalResult, eval},
        parser::ExprValue,
    };

    #[test]
    fn eval_tree_test() {
        let mut fns = HashMap::new();
        fns.insert("test".to_string(), test as EvalFn);
        let mut vars = HashMap::new();
        vars.insert("hello".to_string(), ExprValue::String("world"));
        assert_eq!(
            eval(
                ExprValue::FnCall(
                    "test",
                    vec![ExprValue::Ident("hello"), ExprValue::Number(3.14)]
                ),
                &EvalContext {
                    fns: fns,
                    vars: vars
                }
            )
            .unwrap(),
            EvalResult::String("world 3.14".to_string())
        );
    }
}
