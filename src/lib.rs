use evalexpr::*;

pub fn evaluate(input: &str) -> Result<i64, EvalexprError> {
    eval_int(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        assert_eq!(evaluate("2+2"), "4");
    }

    #[test]
    fn test_bodmas() {
        assert_eq!(evaluate("5+2*3"), "11");
    }

    #[test]
    fn test_all_operators() {
        assert_eq!(evaluate("10+2-3*4/2"), "6");
    }

    #[test]
    fn simple_division() {
        assert_eq!(evaluate("2/1"), "2");
    }
}
