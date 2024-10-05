// Topic: Flow control using if..else if..else
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable is > 5, < 5, or == 5, respectively

// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

use std::{cmp::Ordering, io};

fn get_integer() -> i32 {
    let mut buffer = String::new();
    loop {
        println!("enter an integer: ");
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("please enter the data again");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("only integers allowed");
                buffer.clear();
            }
        }
    }
}

fn integer_compare(a: i32) {
    match a.cmp(&5) {
        Ordering::Less => println!("{:?} is less than 5", a),
        Ordering::Greater => println!("{:?} is greater than 5", a),
        Ordering::Equal => println!("{:?} is equal to 5", a),
    }
}

fn main() {
    integer_compare(get_integer());
}
