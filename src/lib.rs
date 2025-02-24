use evalexpr::*;

pub fn evaluate(input: &str) -> Result<i64, EvalexprError> {
    eval_int(input)
}

