// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

use std::io;

fn get_integer_value() -> i32 {
    let mut value = String::new();
    loop {
        println!("input an integer value: ");
        value.clear();
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only integers allowed");
                continue;
            }
        };
        return value;
    }
}

fn display_message(num: i32) {
    let comparison = num <= 100;
    match comparison {
        true => {
            println!("it's small");
        }
        false => {
            println!("it's big");
        }
    }
}

fn main() {
    let number: i32 = get_integer_value();
    display_message(number);
}
