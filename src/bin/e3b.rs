// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

use std::cmp::Ordering;
use std::io;

fn get_integer_value() -> i32 {
    let mut value = String::new();
    println!("input an integer value: ");
    loop {
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: i32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only integers allowed");
                value.clear();
                continue;
            }
        };
        return value;
    }
}

fn display_msg(value: i32) {
    match value.cmp(&5) {
        Ordering::Less => println!("<5"),
        Ordering::Greater => println!(">5"),
        Ordering::Equal => println!("=5"),
    }
}

fn main() {
    display_msg(get_integer_value());
}
