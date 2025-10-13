use crate::eval::EvalResult;

pub(crate) fn add(args: Vec<EvalResult>) -> EvalResult {
    EvalResult::Number(
        args.iter()
            .map(|val| {
                if let EvalResult::Number(n) = val {
                    *n
                } else {
                    0 as f64
                }
            })
            .reduce(|a, b| a + b)
            .unwrap_or(0 as f64),
    )
}

pub(crate) fn sub(args: Vec<EvalResult>) -> EvalResult {
    EvalResult::Number(
        args.iter()
            .map(|val| {
                if let EvalResult::Number(n) = val {
                    *n
                } else {
                    0 as f64
                }
            })
            .reduce(|a, b| a - b)
            .unwrap_or(0 as f64),
    )
}

pub(crate) fn mul(args: Vec<EvalResult>) -> EvalResult {
    EvalResult::Number(
        args.iter()
            .map(|val| {
                if let EvalResult::Number(n) = val {
                    *n
                } else {
                    0 as f64
                }
            })
            .reduce(|a, b| a * b)
            .unwrap_or(0 as f64),
    )
}

pub(crate) fn div(args: Vec<EvalResult>) -> EvalResult {
    EvalResult::Number(
        args.iter()
            .map(|val| {
                if let EvalResult::Number(n) = val {
                    *n
                } else {
                    0 as f64
                }
            })
            .reduce(|a, b| a / b)
            .unwrap_or(0 as f64),
    )
}

pub(crate) fn pow(args: Vec<EvalResult>) -> EvalResult {
    EvalResult::Number(
        args.iter()
            .map(|val| {
                if let EvalResult::Number(n) = val {
                    *n
                } else {
                    0 as f64
                }
            })
            .reduce(|a, b| a.powf(b))
            .unwrap_or(0 as f64),
    )
}
