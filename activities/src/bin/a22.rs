// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*; // the current file and all its contents

    #[test]
    fn divide_nums() {
        let result = div(4, 2);
        let expected = Some(2);

        assert_eq!(result, expected, "Value should be a decimal");
    }

    #[test]
    fn test_concat() {
        let result = concat("cat", "dog");
        let expect = "catdog".to_owned();

        assert_eq!(result, expect, "Should be catdog");
    }
}