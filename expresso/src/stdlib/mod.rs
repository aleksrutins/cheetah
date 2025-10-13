use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::eval::EvalFn;

mod bool;
mod num;
mod quote;
mod string;

lazy_static! {
    pub static ref STDLIB: HashMap<String, EvalFn> = {
        let mut m = HashMap::new();

        m.insert("concat".to_string(), string::concat as EvalFn);

        m.insert("add".to_string(), num::add as EvalFn);
        m.insert("sub".to_string(), num::sub as EvalFn);
        m.insert("mul".to_string(), num::mul as EvalFn);
        m.insert("div".to_string(), num::div as EvalFn);
        m.insert("pow".to_string(), num::pow as EvalFn);

        m.insert("if".to_string(), bool::if_ as EvalFn);

        m.insert("call".to_string(), quote::call as EvalFn);
        m.insert("let".to_string(), quote::let_ as EvalFn);

        m
    };
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
    fn test_add() {
        let ctx = EvalContext {
            vars: HashMap::new(),
            fns: STDLIB.clone(),
        };
        assert_eq!(
            eval_str("add(1.1, 2.2, 3.3)", ctx).unwrap(),
            EvalResult::Number(6.6)
        );
    }
}
