// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

use std::io;

fn get_integer_input() -> i32 {
    let mut input = String::new();
    loop {
        println!("input an integer: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line!");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only integers allowed!");
                input.clear();
                continue;
            }
        };
        return input;
    }
}

fn compare(num: i32) -> &'static str {
    match num {
        1 => {
            return "one";
        }
        2 => {
            return "two";
        }
        3 => {
            return "three";
        }
        _ => return "other",
    }
}

fn main() {
    let result: &str = compare(get_integer_input());
    println!("{result}");
}
