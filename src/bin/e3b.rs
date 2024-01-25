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
    let mut buffer = String::new();
    println!("enter an integer: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("please enter your data again");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                buffer.clear();
                println!("please enter only integers");
                continue;
            }
        }
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
