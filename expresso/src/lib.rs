use crate::{
    eval::{EvalContext, EvalResult},
    parser::parse_expr,
};
use std::error::Error;

pub mod eval;
pub mod parser;
pub mod stdlib;

pub fn eval_str<'a>(expr: &'a str, ctx: EvalContext<'a>) -> Result<EvalResult<'a>, Box<dyn Error>> {
    Ok(crate::eval::eval(parse_expr(expr)?, ctx)?)
}

#[cfg(test)]
mod tests {
    use crate::{
        eval::{EvalContext, EvalFn, EvalResult, test},
        eval_str,
        parser::ExprValue,
    };
    use std::collections::HashMap;

    #[test]
    pub fn everything_together() {
        let mut fns = HashMap::new();
        fns.insert("test".to_string(), test as EvalFn);
        let mut vars = HashMap::new();
        vars.insert("hello".to_string(), ExprValue::String("world".into()));
        assert_eq!(
            eval_str("test(hello, 3.14)", EvalContext { fns, vars }).unwrap(),
            EvalResult::String("world 3.14".to_string())
        );
    }

    #[test]
    fn test_splat() {
        let mut fns = HashMap::new();
        fns.insert("test".to_string(), test as EvalFn);
        let mut vars = HashMap::new();
        vars.insert(
            "hello".to_string(),
            ExprValue::Array(vec![
                ExprValue::String("world".into()),
                ExprValue::Number(3.14),
            ]),
        );
        assert_eq!(
            eval_str("test(...hello)", EvalContext { fns, vars }).unwrap(),
            EvalResult::String("world 3.14".to_string())
        );
    }
}
