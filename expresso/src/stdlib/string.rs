use crate::eval::{EvalFn, EvalResult};

pub(crate) fn concat(args: Vec<EvalResult>) -> EvalResult {
    EvalResult::String(
        args.iter()
            .map(|val| format!("{}", val))
            .collect::<Vec<_>>()
            .concat(),
    )
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
    fn test_concat() {
        let ctx = EvalContext {
            vars: HashMap::new(),
            fns: STDLIB.clone(),
        };
        assert_eq!(
            eval_str("concat(\"hello\", \" \", \"world\", 3)", ctx).unwrap(),
            EvalResult::String("hello world3".to_string())
        );
    }
}
