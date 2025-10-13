use crate::eval::{EvalResult, eval};

pub(crate) fn if_(args: Vec<EvalResult>) -> EvalResult {
    if let Some(EvalResult::Boolean(condition)) = args.get(0)
        && let Some(EvalResult::Quoted(then, ctx)) = args.get(1)
    {
        if *condition {
            eval(then.as_ref().clone(), ctx.clone()).unwrap_or(EvalResult::None)
        } else {
            if let Some(EvalResult::Quoted(else_, ctx)) = args.get(2) {
                eval(else_.as_ref().clone(), ctx.clone()).unwrap_or(EvalResult::None)
            } else {
                EvalResult::None
            }
        }
    } else {
        EvalResult::None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        eval::{EvalContext, EvalResult},
        eval_str,
        stdlib::STDLIB,
    };

    #[test]
    fn test_if() {
        let ctx = EvalContext {
            vars: HashMap::new(),
            fns: STDLIB.clone(),
        };
        assert_eq!(
            eval_str("if(true, 'add(2, 2), 'add(4, 4))", ctx.clone()).unwrap(),
            EvalResult::Number(4 as f64)
        );
        assert_eq!(
            eval_str("if(false, 'add(2, 2), 'add(4, 4))", ctx).unwrap(),
            EvalResult::Number(8 as f64)
        );
    }
}
