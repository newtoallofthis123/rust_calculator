use evalexpr::*;
use wasm_bindgen::prelude::*;

/// evaluate the input string and return the result
/// Since it makes error handling easier, we return a string instead of an actual number
#[wasm_bindgen]
pub fn evaluate(input: &str) -> String {
    match eval(input) {
        Ok(result) => result.to_string(),
        Err(e) => e.to_string(),
    }
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
