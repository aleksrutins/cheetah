use crate::eval::{EvalContext, EvalResult, eval};

pub(crate) fn call<'a>(args: Vec<EvalResult<'a>>) -> EvalResult<'a> {
    if let Some(EvalResult::Quoted(q, ctx)) = args.get(0) {
        let args_list = if let Some(EvalResult::Object(args_obj)) = args.get(1) {
            args_obj.clone()
        } else if let Some(EvalResult::Array(args_arr)) = args.get(1) {
            args_arr
                .iter()
                .map(|v| v.clone())
                .enumerate()
                .map(|(k, v)| (format!("${}", k), v))
                .collect::<Vec<_>>()
        } else {
            return EvalResult::None;
        };
        let mut new_ctx: EvalContext<'a> = EvalContext {
            fns: ctx.fns.clone(),
            vars: ctx.vars.clone(),
        };
        for arg in args_list {
            new_ctx.vars.insert(arg.0.clone(), arg.1.clone().into());
        }
        eval(q.as_ref().clone(), new_ctx)
            .unwrap_or(EvalResult::None)
            .to_owned()
    } else {
        EvalResult::None
    }
}

pub(crate) fn let_(args: Vec<EvalResult>) -> EvalResult {
    call(vec![
        args.get(1).unwrap().clone(),
        args.get(0).unwrap().clone(),
    ])
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
    fn test_let_call() {
        let ctx = EvalContext {
            vars: HashMap::new(),
            fns: STDLIB.clone(),
        };
        assert_eq!(
            eval_str(r#"let({"a":2,"b":4}, 'add(a, b))"#, ctx.clone()).unwrap(),
            EvalResult::Number(6 as f64)
        );
        assert_eq!(
            eval_str(r#"call('add(a, b), {"a": 2, "b": 4})"#, ctx.clone()).unwrap(),
            EvalResult::Number(6 as f64)
        );
        assert_eq!(
            eval_str(
                r#"
                let({"fn": 'add(a, b)},
                    'call(fn, {"a": 2, "b": 4}))
                    "#,
                ctx.clone()
            )
            .unwrap(),
            EvalResult::Number(6 as f64)
        );

        assert_eq!(
            eval_str(
                r#"
            call('add($0, $1), [2, 2])
            "#,
                ctx.clone()
            )
            .unwrap(),
            EvalResult::Number(4 as f64)
        )
    }
}
