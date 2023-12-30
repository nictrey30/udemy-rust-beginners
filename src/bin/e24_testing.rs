#![allow(dead_code, unused_variables, unused_imports)]
// Topic: Testing
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to dgaaetermine how the they should operate.

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
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp_lower() {
        let result_lower = clamp(6, 7, 100);
        let result_higher = clamp(101, 7, 100);
        let result_middle = clamp(5, 2, 10);
        assert_eq!(result_lower, 7, "n=6 should be clamped into lower limit=7");
        assert_eq!(
            result_higher, 100,
            "n=101 should be clamped into higher limit=100"
        );
        assert_eq!(result_middle, 5, "n=5 should be returned");
    }

    #[test]
    fn check_clamp_higher() {
        let result_higher = clamp(101, 7, 100);
        assert_eq!(
            result_higher, 100,
            "n=101 should be clamped into higher limit=100"
        );
    }

    #[test]
    fn check_clamp_middle() {
        let result_middle = clamp(5, 2, 10);
        assert_eq!(result_middle, 5, "n=5 should be returned");
    }
}
