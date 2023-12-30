#![allow(dead_code, unused_variables, unused_imports)]
fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {}

// the macro will tell the compiler that the module is specifically for testing
#[cfg(test)]
mod test {
    use crate::all_caps;

    // if the program is aborted the test fails, otherwise it succeeds
    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        // assert_eq! macro takes 3 arg: 1. value we want to check, 2. value that we are expecting, 3. msg when the test does not work
        assert_eq!(result, expected, "string should be all uppercase");
    }
}
